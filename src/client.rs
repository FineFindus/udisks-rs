use zbus::{fdo::ObjectManagerProxy, zvariant::OwnedObjectPath};

use crate::{block, job, manager, object::Object, partition, partitiontable};

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

    /// Gets a human-readable and localized text string describing the operation of job.
    ///
    /// For known job types, see the documentation for [`job::JobProxy::operation`].
    pub async fn job_description(&self, job: job::JobProxy<'_>) -> zbus::Result<String> {
        Ok(self.job_description_from_operation(&job.operation().await?))
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
}
