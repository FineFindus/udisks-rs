use zbus::{fdo::ObjectManagerProxy, names::OwnedInterfaceName, zvariant::OwnedObjectPath};

use crate::{
    block::{self, BlockProxy},
    drive, job, manager,
    object::Object,
    partition, partition_types, partitiontable,
};

/// Utility routines for accessing the UDisks service
pub struct Client {
    connection: zbus::Connection,
    object_manager: zbus::fdo::ObjectManagerProxy<'static>,
    manager: manager::ManagerProxy<'static>,
}

impl Client {
    /// Create a new client.
    pub async fn new() -> zbus::Result<Self> {
        let connection = zbus::Connection::system().await?;
        Self::new_for_connection(connection).await
    }

    /// Creates a new client based on the given [`zbus::Connection`].
    pub async fn new_for_connection(connection: zbus::Connection) -> zbus::Result<Self> {
        let object_manager = ObjectManagerProxy::builder(&connection)
            .destination("org.freedesktop.UDisks2")
            .unwrap()
            .path("/org/freedesktop/UDisks2")
            .unwrap()
            .build()
            .await?;

        let manager = manager::ManagerProxy::new(&connection).await?;

        Ok(Self {
            connection,
            object_manager,
            manager,
        })
    }

    /// Returns the [`zbus::fdo::ObjectManagerProxy`] used by the [Client].
    pub fn object_manager(&self) -> &zbus::fdo::ObjectManagerProxy<'_> {
        &self.object_manager
    }

    /// Returns a reference to the manager interface.
    pub fn manager(&self) -> &manager::ManagerProxy<'_> {
        &self.manager
    }

    /// Convenience function for looking up an [Object] for `object_path`.
    ///
    /// # Errors
    /// Returns an error if the given object path cannot be converted to an [zbus::zvariant::OwnedObjectPath]
    pub fn object<P: TryInto<OwnedObjectPath>>(&self, object_path: P) -> Result<Object, P::Error> {
        let path = object_path.try_into()?;
        Ok(Object::new(
            path,
            self.object_manager.clone(),
            self.connection.clone(),
        ))
    }

    /// Gets all  the [`job::JobProxy`] instances for the given object.
    ///
    /// If no instances are found, the returned vector is empty.
    pub async fn jobs_for_object(&self, object: Object) -> Vec<OwnedObjectPath> {
        //TODO: maybe this should be moved to object directly?
        let object_path = object.object_path();

        let mut blocks = Vec::new();

        for object in self
            .object_manager
            .get_managed_objects()
            .await
            .into_iter()
            .flatten()
            .filter_map(|(object_path, _)| self.object(object_path).ok())
        {
            let Ok(job) = object.job().await else {
                continue;
            };

            blocks.extend(
                job.objects()
                    .await
                    .into_iter()
                    .flatten()
                    .filter(|job_object_path| job_object_path == object_path),
            );
        }
        blocks
    }

    /// Gets a human-readable and localized text string describing the operation of job.
    ///
    /// For known job types, see the documentation for [`job::JobProxy::operation`].
    pub async fn job_description(&self, job: job::JobProxy<'_>) -> zbus::Result<String> {
        Ok(self.job_description_from_operation(&job.operation().await?))
    }

    /// Gets the [`block::BlockProxy`] for the given `block_device_number`.
    ///
    /// If no block is found, [`None`] is returned,
    pub async fn block_for_dev(&self, block_device_number: u64) -> Option<block::BlockProxy> {
        for object in self
            .object_manager
            .get_managed_objects()
            .await
            .into_iter()
            .flatten()
            .filter_map(|(object_path, _)| self.object(object_path).ok())
        {
            let Ok(block) = object.block().await else {
                continue;
            };

            if Ok(block_device_number) == block.device_number().await {
                return Some(block);
            }
        }
        None
    }

    /// Gets all  the [`block::BlockProxy`] instances with the given label.
    ///
    /// If no instances are found, the returned vector is empty.
    pub async fn block_for_label(&self, label: &str) -> Vec<block::BlockProxy> {
        //TODO refactor once it is possible to use iterators with async

        let mut blocks = Vec::new();

        for object in self
            .object_manager
            .get_managed_objects()
            .await
            .into_iter()
            .flatten()
            .filter_map(|(object_path, _)| self.object(object_path).ok())
        {
            let Ok(block) = object.block().await else {
                continue;
            };

            if Ok(label) == block.id_label().await.as_deref() {
                blocks.push(block);
            }
        }
        blocks
    }

    /// Gets all the [`block::BlockProxy`]s for the given `uuid`.
    ///
    /// If no blocks are found, the returned vector is empty.
    pub async fn block_for_uuid(&self, uuid: &str) -> Vec<block::BlockProxy> {
        let mut blocks = Vec::new();
        for object in self
            .object_manager
            .get_managed_objects()
            .await
            .into_iter()
            .flatten()
            .filter_map(|(object_path, _)| self.object(object_path).ok())
        {
            let Ok(block) = object.block().await else {
                continue;
            };

            if Ok(uuid) == block.id_uuid().await.as_deref() {
                blocks.push(block);
            }
        }
        blocks
    }

    /// Returns all top-level [`Object`]s for the given drive.
    ///
    /// Top-level blocks are blocks that do not have a partition associated with it.
    async fn top_level_blocks_for_drive(&self, drive_object_path: &OwnedObjectPath) -> Vec<Object> {
        let mut blocks = Vec::new();
        for object in self
            .object_manager
            .get_managed_objects()
            .await
            .into_iter()
            .flatten()
            .filter_map(|(object_path, _)| self.object(object_path).ok())
        {
            let Ok(block) = object.block().await else {
                continue;
            };

            if block.drive().await.as_deref() == Ok(drive_object_path)
                && object.partition().await.is_err()
            {
                blocks.push(object);
            }
        }
        blocks
    }

    async fn object_for_interface<P: TryInto<OwnedInterfaceName>>(
        &self,
        interface: P,
    ) -> zbus::Result<Object> {
        let managed_objects = self.object_manager.get_managed_objects().await?;

        let interface = interface
            .try_into()
            .map_err(|_| zbus::Error::InterfaceNotFound)?;

        managed_objects
            .into_iter()
            .filter(|(_, interfaces)| interfaces.contains_key(&interface))
            .find_map(|(path, _)| self.object(path).ok())
            .ok_or(zbus::Error::InterfaceNotFound)
    }

    /// Gets the [`block::BlockProxy`], if exists, for the given [`drive::DriveProxy`]
    ///
    /// The returned block is for the whole disk drive, so [`partition::PartitionProxy`] is never
    /// returned.
    ///
    /// If `physical` is set to true, a block that is able to send low-level SCSI commands is
    /// returned. If `physical` is set to false, a block device that can read/write data is
    /// returned.
    pub async fn block_for_drive(
        &self,
        drive: drive::DriveProxy<'_>,
        _physical: bool,
    ) -> Option<block::BlockProxy> {
        let object = self
            .object_for_interface(drive.interface().clone())
            .await
            .ok()?;

        for object in self
            .top_level_blocks_for_drive(object.object_path())
            .await
            .iter()
        {
            if let Ok(block) = object.block().await {
                return Some(block);
            };
        }
        None
    }

    /// Gets the [`drive::DriveProxy`] for the given [`block::BlockProxy`].
    ///
    /// If no block is found, [`None`] is returned.
    pub async fn drive_for_block(
        &self,
        block: BlockProxy<'_>,
    ) -> zbus::Result<drive::DriveProxy<'static>> {
        let drive = block.drive().await?;
        self.object(drive)?.drive().await
    }

    /// If the given [`block::BlockProxy`] is an encrypted device, returns the cleartext device.
    ///
    /// If no block is found, [`None`] is returned.
    pub async fn cleartext_block(&self, block: BlockProxy<'_>) -> Option<block::BlockProxy<'_>> {
        let object_path = block.path().to_owned().into();
        for object in self
            .object_manager
            .get_managed_objects()
            .await
            .into_iter()
            .flatten()
            .filter_map(|(object_path, _)| self.object(object_path).ok())
        {
            let Ok(block) = object.block().await else {
                continue;
            };
            if block.crypto_backing_device().await.as_ref() == Ok(&object_path) {
                return Some(block);
            }
        }
        None
    }

    /// Returns the [`partitiontable::PartitionTableProxy`] for the given partition.
    ///
    /// # Errors
    /// Returns an error if it is unable to get the table or the [`Object`] for the table.
    pub async fn partition_table(
        &self,
        partition: partition::PartitionProxy<'_>,
    ) -> zbus::Result<partitiontable::PartitionTableProxy<'_>> {
        //TODO: C version docs do not mention that it can return NULL?
        //https://github.com/storaged-project/udisks/blob/4f24c900383d3dc28022f62cab3eb434d19b6b82/udisks/udisksclient.c#L1429
        self.object(partition.table().await?)?
            .partition_table()
            .await
    }

    /// Gets a human-readable and localized text string describing the operation of job.
    ///
    /// For known job types, see the documentation for [`job::JobProxy::operation`].
    pub fn job_description_from_operation(&self, operation: &str) -> String {
        //TODO use gettext to translate the strings
        match operation {
            "ata-smart-selftest" => String::from("SMART self-test"),
            "drive-eject" => String::from("Ejecting Medium"),
            "encrypted-unlock" => String::from("Unlocking Device"),
            "encrypted-lock" => String::from("Locking Device"),
            "encrypted-modify" => String::from("Modifying Encrypted Device"),
            "encrypted-resize" => String::from("Resizing Encrypted Device"),
            "swapspace-start" => String::from("Starting Swap Device"),
            "swapspace-stop" => String::from("Stopping Swap Device"),
            "swapspace-modify" => String::from("Modifying Swap Device"),
            "filesystem-check" => String::from("Checking Filesystem"),
            "filesystem-mount" => String::from("Mounting Filesystem"),
            "filesystem-unmount" => String::from("Unmounting Filesystem"),
            "filesystem-modify" => String::from("Modifying Filesystem"),
            "filesystem-repair" => String::from("Repairing Filesystem"),
            "filesystem-resize" => String::from("Resizing Filesystem"),
            "format-erase" => String::from("Erasing Device"),
            "format-mkfs" => String::from("Creating Filesystem"),
            "loop-setup" => String::from("Setting Up Loop Device"),
            "partition-modify" => String::from("Modifying Partition"),
            "partition-delete" => String::from("Deleting Partition"),
            "partition-create" => String::from("Creating Partition"),
            "cleanup" => String::from("Cleaning Up"),
            "ata-secure-erase" => String::from("ATA Secure Erase"),
            "ata-enhanced-secure-erase" => String::from("ATA Enhanced Secure Erase"),
            "md-raid-stop" => String::from("Stopping RAID Array"),
            "md-raid-start" => String::from("Starting RAID Array"),
            "md-raid-fault-device" => String::from("Marking Device as Faulty"),
            "md-raid-remove-device" => String::from("Removing Device from Array"),
            "md-raid-add-device" => String::from("Adding Device to Array"),
            "md-raid-set-bitmap" => String::from("Setting Write-Intent Bitmap"),
            "md-raid-create" => String::from("Creating RAID Array"),
            _ => format!("Unknown ({})", operation),
        }
    }

    /// Returns, if exists, the human-readable localized name of the [PartitionType].
    pub fn partition_type_for_display(
        &self,
        partition_table_type: &str,
        partition_type: &str,
    ) -> Option<&'static str> {
        partition_types::PARTITION_TYPES
            .iter()
            .find(|pt| pt.table_type == partition_table_type && pt.ty == partition_type)
            //TODO: C version calls gettext here
            //https://github.com/storaged-project/udisks/blob/4f24c900383d3dc28022f62cab3eb434d19b6b82/udisks/udisksclient.c#L2653C26-L2653C26
            .map(|partition_type| partition_type.name)
    }
}
