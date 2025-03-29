use gettextrs::pgettext;
use zbus::{fdo::ObjectManagerProxy, zvariant::OwnedObjectPath};

use crate::{
    block::{self, BlockProxy},
    drive, error,
    gettext::{dpgettext, pgettext_f},
    id::ID_TYPES,
    job, r#loop, manager, mdraid,
    object::Object,
    object_info::ObjectInfo,
    partition, partition_subtypes,
    partition_types::{self, PARTITION_TYPES, PartitionTypeInfo},
    partitiontable,
};

const KILOBYTE_FACTOR: f64 = 1000.0;
const MEGABYTE_FACTOR: f64 = 1000.0 * 1000.0;
const GIGABYTE_FACTOR: f64 = 1000.0 * 1000.0 * 1000.0;
const TERABYTE_FACTOR: f64 = 1000.0 * 1000.0 * 1000.0 * 1000.0;

const KIBIBYTE_FACTOR: f64 = 1024.0;
const MEBIBYTE_FACTOR: f64 = 1024.0 * 1024.0;
const GIBIBYTE_FACTOR: f64 = 1024.0 * 1024.0 * 1024.0;
const TEBIBYTE_FACTOR: f64 = 1024.0 * 1024.0 * 1024.0 * 10242.0;

/// Utility routines for accessing the UDisks service.
///
/// It should be used for accessing the UDisks service from a client program.
#[derive(Debug, Clone)]
pub struct Client {
    connection: zbus::Connection,
    object_manager: zbus::fdo::ObjectManagerProxy<'static>,
    manager: manager::ManagerProxy<'static>,
}

impl Client {
    /// Create a new client.
    pub async fn new() -> error::Result<Self> {
        let connection = zbus::Connection::system().await?;
        Self::new_for_connection(connection).await
    }
    /// Creates a new client based on the given [`zbus::Connection`].
    pub async fn new_for_connection(connection: zbus::Connection) -> error::Result<Self> {
        let object_manager = ObjectManagerProxy::builder(&connection)
            .destination("org.freedesktop.UDisks2")?
            .path("/org/freedesktop/UDisks2")?
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
    pub async fn jobs_for_object(&self, object: &Object) -> Vec<OwnedObjectPath> {
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
    pub fn job_description_from_operation(&self, operation: &str) -> String {
        match operation {
            "ata-smart-selftest" => pgettext("job", "SMART self-test"),
            "drive-eject" => pgettext("job", "Ejecting Medium"),
            "encrypted-unlock" => pgettext("job", "Unlocking Device"),
            "encrypted-lock" => pgettext("job", "Locking Device"),
            "encrypted-modify" => pgettext("job", "Modifying Encrypted Device"),
            "encrypted-resize" => pgettext("job", "Resizing Encrypted Device"),
            "swapspace-start" => pgettext("job", "Starting Swap Device"),
            "swapspace-stop" => pgettext("job", "Stopping Swap Device"),
            "swapspace-modify" => pgettext("job", "Modifying Swap Device"),
            "filesystem-check" => pgettext("job", "Checking Filesystem"),
            "filesystem-mount" => pgettext("job", "Mounting Filesystem"),
            "filesystem-unmount" => pgettext("job", "Unmounting Filesystem"),
            "filesystem-modify" => pgettext("job", "Modifying Filesystem"),
            "filesystem-repair" => pgettext("job", "Repairing Filesystem"),
            "filesystem-resize" => pgettext("job", "Resizing Filesystem"),
            "format-erase" => pgettext("job", "Erasing Device"),
            "format-mkfs" => pgettext("job", "Creating Filesystem"),
            "loop-setup" => pgettext("job", "Setting Up Loop Device"),
            "partition-modify" => pgettext("job", "Modifying Partition"),
            "partition-delete" => pgettext("job", "Deleting Partition"),
            "partition-create" => pgettext("job", "Creating Partition"),
            "cleanup" => pgettext("job", "Cleaning Up"),
            "ata-secure-erase" => pgettext("job", "ATA Secure Erase"),
            "ata-enhanced-secure-erase" => pgettext("job", "ATA Enhanced Secure Erase"),
            "md-raid-stop" => pgettext("job", "Stopping RAID Array"),
            "md-raid-start" => pgettext("job", "Starting RAID Array"),
            "md-raid-fault-device" => pgettext("job", "Marking Device as Faulty"),
            "md-raid-remove-device" => pgettext("job", "Removing Device from Array"),
            "md-raid-add-device" => pgettext("job", "Adding Device to Array"),
            "md-raid-set-bitmap" => pgettext("job", "Setting Write-Intent Bitmap"),
            "md-raid-create" => pgettext("job", "Creating RAID Array"),
            _ => pgettext_f("unknown-job", "Unknown ({})", [operation]),
        }
    }

    /// Gets a human-readable and localized text string describing the operation of job.
    ///
    /// For known job types, see the documentation for [`job::JobProxy::operation`].
    pub async fn job_description(&self, job: &job::JobProxy<'_>) -> error::Result<String> {
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

    /// Gets all the [`block::BlockProxy`] instances with the given label.
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
        drive: &drive::DriveProxy<'_>,
        _physical: bool,
    ) -> Option<block::BlockProxy> {
        let object = self.object(drive.inner().path().clone()).ok()?;

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

    /// Gets the [`drive::DriveProxy`] for the given [`block::BlockProxy`], if any.
    ///
    /// # Errors
    /// If no drive is found, [`zbus::Error::InterfaceNotFound`] is returned.
    pub async fn drive_for_block(
        &self,
        block: &block::BlockProxy<'_>,
    ) -> error::Result<drive::DriveProxy<'static>> {
        let drive = block.drive().await?;
        self.object(drive)?.drive().await
    }

    /// If the given [`block::BlockProxy`] is an encrypted device, returns the cleartext device.
    ///
    /// If no block is found, [`None`] is returned.
    pub async fn cleartext_block(
        &self,
        block: &block::BlockProxy<'_>,
    ) -> Option<block::BlockProxy<'_>> {
        let object_path = block.inner().path().to_owned().into();
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
        partition: &partition::PartitionProxy<'_>,
    ) -> error::Result<partitiontable::PartitionTableProxy<'_>> {
        self.object(partition.table().await?)?
            .partition_table()
            .await
    }

    /// Returns the [`loop::LoopProxy`] for the given [`block::BlockProxy`].
    ///
    /// This only works if the block is a loop device, or a partition of a loop device.
    ///
    /// # Errors
    /// Returns an error if it is unable to get the loop interface.
    pub async fn loop_for_block(
        &self,
        block: &block::BlockProxy<'_>,
    ) -> error::Result<r#loop::LoopProxy> {
        let object = self.object(block.inner().path().clone())?;

        if let Ok(loop_proxy) = object.r#loop().await {
            return Ok(loop_proxy);
        }

        // possibly partition of a loop device
        let partition = object.partition().await?;
        let partitiontable = self.partition_table(&partition).await?;
        let partitiontable_object = self.object(partitiontable.inner().path().clone())?;
        partitiontable_object.r#loop().await
    }

    /// Returns all [`partition::PartitionProxy`] of the given [`partitiontable::PartitionTableProxy`].
    pub async fn partitions(
        &self,
        table: &partitiontable::PartitionTableProxy<'_>,
    ) -> Vec<partition::PartitionProxy<'_>> {
        let mut partitions = Vec::new();
        // safe to unwrap as the table's object path does not need to be converted
        let table_object = self.object(table.inner().path().clone()).unwrap();
        let table_object_path = table_object.object_path();

        for object in self
            .object_manager
            .get_managed_objects()
            .await
            .into_iter()
            .flatten()
            .filter_map(|(object_path, _)| self.object(object_path).ok())
        {
            let Ok(partition) = object.partition().await else {
                continue;
            };

            if partition.table().await.as_ref() == Ok(table_object_path) {
                partitions.push(partition);
            }
        }
        partitions
    }

    /// Returns all [`partition::PartitionProxy`] of the given [`partitiontable::PartitionTableProxy`].
    pub async fn drive_siblings(
        &self,
        drive: &drive::DriveProxy<'_>,
    ) -> Vec<drive::DriveProxy<'_>> {
        let mut drive_siblings = Vec::new();
        let sibling_id = drive.sibling_id().await;

        if sibling_id.is_err() || sibling_id.as_ref().unwrap().is_empty() {
            return drive_siblings;
        }

        for object in self
            .object_manager
            .get_managed_objects()
            .await
            .into_iter()
            .flatten()
            .filter_map(|(object_path, _)| self.object(object_path).ok())
        {
            let Ok(iter_drive) = object.drive().await else {
                continue;
            };

            if
            // TODO: C version checks if we're the same drive
            // rust version doesn't implement partial_cmp
            // iter_drive != drive &&
            iter_drive.sibling_id().await.as_ref() == sibling_id.as_ref() {
                drive_siblings.push(iter_drive);
            }
        }
        drive_siblings
    }

    async fn block_or_blocks_for_mdraid(
        &self,
        mdraid: &mdraid::MDRaidProxy<'_>,
        //TODO: pass in a function
        // member_get: impl Fn(&block::BlockProxy<'a>) -> Future<Output = error::Result<OwnedObjectPath>> + 'a,
        members: bool,
        only_first_one: bool,
        skip_partitions: bool,
    ) -> Vec<block::BlockProxy> {
        let mut blocks = Vec::new();
        // safe to unwrap as the table's object path does not need to be converted
        let raid_object = self.object(mdraid.inner().path().clone()).unwrap();

        let raid_objpath = raid_object.object_path();

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

            // skip partitions
            if skip_partitions && object.partition().await.is_ok() {
                continue;
            }

            // if member_get(&block).await.as_ref() == Ok(raid_objpath) {
            let block_objpath = if members {
                block.mdraid().await
            } else {
                block.mdraid_member().await
            };

            if block_objpath.as_ref() == Ok(raid_objpath) {
                blocks.push(block);
                if only_first_one {
                    break;
                }
            }
        }

        blocks
    }

    /// Returns the RAID device (e.g. `/dev/md0`) for the given mdraid.
    ///
    /// In the case of a [split-brain syndrome](https://en.wikipedia.org/wiki/Split-brain_(computing)),
    /// it is undefined which RAID device is returned. For example this can happen if `/dev/sda` and `/dev/sdb`
    /// are components of a two-disk RAID-1 and `/dev/md0` and `/dev/md1` are two degraded arrays,
    /// each one using exactly one of the two devices. Use [`Client::all_blocks_for_mdraid`] to get all RAID devices.
    ///
    /// If no RAID device is running, [`Option::None`] is returned.
    pub async fn block_for_mdraid(
        &self,
        mdraid: &mdraid::MDRaidProxy<'_>,
    ) -> Option<BlockProxy<'_>> {
        self.block_or_blocks_for_mdraid(mdraid, false, true, true)
            .await
            .first()
            .cloned()
    }

    /// Returns all RAID devices (e.g. `/dev/md0` and `/dev/md1`) for the given mdraid.
    ///
    /// This is usually only useful [split-brain syndrome](https://en.wikipedia.org/wiki/Split-brain_(computing)),
    /// and is normally used only to convey the problem in an user interface. See [`Client::block_for_mdraid`] for an example.
    pub async fn all_blocks_for_mdraid(
        &self,
        mdraid: &mdraid::MDRaidProxy<'_>,
    ) -> Vec<block::BlockProxy<'_>> {
        self.block_or_blocks_for_mdraid(mdraid, false, false, true)
            .await
    }

    /// returns the physical block devices that are part of the given raid.
    pub async fn members_for_mdraid(
        &self,
        mdraid: &mdraid::MDRaidProxy<'_>,
    ) -> Vec<block::BlockProxy<'_>> {
        self.block_or_blocks_for_mdraid(mdraid, true, false, false)
            .await
    }

    /// Returns the [`mdraid::MDRaidProxy`] that the given block is the block device for.
    ///
    /// # Errors
    /// Returns an error if no [`mdraid::MDRaidProxy`] for the block is found, or the block is not
    /// a MD-RAID block device.
    pub async fn mdraid_for_block(
        &self,
        block: &block::BlockProxy<'_>,
    ) -> error::Result<mdraid::MDRaidProxy<'_>> {
        let object = self.object(block.mdraid().await?)?;
        object.mdraid().await
    }

    /// Returns information about the given object for presentation in a user information.
    ///
    /// The returned information is localized.
    pub async fn object_info<'a>(&self, object: &'a Object) -> ObjectInfo<'a> {
        let mut object_info = ObjectInfo::new(object);

        //populate object_info
        if let Ok(drive) = object.drive().await {
            object_info.info_for_drive(self, &drive, None).await;
        } else if let Ok(mdraid) = object.mdraid().await {
            object_info.info_for_mdraid(self, mdraid, None).await;
        } else if let Ok(block) = object.block().await {
            let partition = object.partition().await;

            let drive = self.drive_for_block(&block);
            if let Ok(drive) = drive.await {
                object_info
                    .info_for_drive(self, &drive, partition.ok())
                    .await;
                return object_info;
            }

            let mdraid = self.mdraid_for_block(&block);
            if let Ok(mdraid) = mdraid.await {
                object_info
                    .info_for_mdraid(self, mdraid, partition.ok())
                    .await;
                return object_info;
            }

            if let Ok(loop_proxy) = object.r#loop().await {
                object_info
                    .info_for_loop(self, loop_proxy, block, partition.ok())
                    .await;
            } else {
                object_info
                    .info_for_block(self, block, partition.ok())
                    .await;
            }
        }

        object_info
    }

    /// Returns informating about the given partition that is suitable for presentation in an user
    /// interface in a single line of text.
    ///
    /// The returned string is localized and includes things like the partition type, flags (if
    /// any) and name (if any).
    ///
    /// # Errors
    /// Returns an errors if it fails to read any of the aforementioned information.
    pub async fn partition_info(
        &self,
        partition: &partition::PartitionProxy<'_>,
    ) -> error::Result<String> {
        let flags = partition.flags().await?;
        let table = self.partition_table(partition).await?;
        let mut flags_str = String::new();

        match table.type_().await.as_deref() {
            Ok("dos") if flags.contains(partition::PartitionFlags::Bootable) => {
                // Translators: Corresponds to the DOS/Master-Boot-Record "bootable" flag for a partition
                flags_str.push_str(&format!(", {}", pgettext("dos-part-flag", "Bootable")))
            }
            Ok("gpt") => {
                let flag_map = [
                    (
                        partition::PartitionFlags::SystemPartition,
                        // Translators: Corresponds to the GPT "system" flag for a partition,
                        // see http://en.wikipedia.org/wiki/GUID_Partition_Table
                        pgettext("gpt-part-flag", "System"),
                    ),
                    (
                        partition::PartitionFlags::LegacyBIOSBootable,
                        // Translators: Corresponds to the GPT "legacy bios bootable" flag for a partition,
                        // see http://en.wikipedia.org/wiki/GUID_Partition_Table
                        pgettext("gpt-part-flag", "Legacy BIOS Bootable"),
                    ),
                    (
                        partition::PartitionFlags::ReadOnly,
                        // Translators: Corresponds to the GPT "read-only" flag for a partition,
                        // see http://en.wikipedia.org/wiki/GUID_Partition_Table
                        pgettext("gpt-part-flag", "Read-only"),
                    ),
                    (
                        partition::PartitionFlags::Hidden,
                        // Translators: Corresponds to the GPT "hidden" flag for a partition,
                        // see http://en.wikipedia.org/wiki/GUID_Partition_Table
                        pgettext("gpt-part-flag", "Hidden"),
                    ),
                    (
                        partition::PartitionFlags::NoAutoMount,
                        // Translators: Corresponds to the GPT "no automount" flag for a partition,
                        // see http://en.wikipedia.org/wiki/GUID_Partition_Table
                        pgettext("gpt-part-flag", "No Automaount"),
                    ),
                ];

                for (flag, info) in flag_map {
                    if flags.contains(flag) {
                        flags_str.push_str(&format!(", {}", info));
                    }
                }
            }
            _ => {}
        };
        let type_str = match self
            .partition_type_for_display(&table.type_().await?, &partition.type_().await?)
        {
            Some(val) => val.to_owned(),
            _ => partition.type_().await?,
        };

        let partition_info;
        if !flags_str.is_empty() {
            // Translators: Partition info. First {} is the type, second {} is a list of flags
            partition_info = pgettext_f("partition-info", "{} ({})", [type_str, flags_str])
        } else if type_str.is_empty() {
            // Translators: The Partition info when unknown
            partition_info = pgettext("partition-info", "Unknown")
        } else {
            partition_info = type_str;
        }

        Ok(partition_info)
    }

    fn pow2_size(&self, size: u64) -> String {
        //TODO: refactor
        let size = size as f64;

        let display_size;
        let unit;
        if size < MEBIBYTE_FACTOR {
            display_size = size / KIBIBYTE_FACTOR;
            /* Translators: SI prefix and standard unit symbol, translate cautiously (or not at all) */
            unit = pgettext("byte-size-pow2", "KiB");
        } else if size < GIBIBYTE_FACTOR {
            display_size = size / MEBIBYTE_FACTOR;
            /* Translators: SI prefix and standard unit symbol, translate cautiously (or not at all) */
            unit = pgettext("byte-size-pow2", "MiB");
        } else if size < TEBIBYTE_FACTOR {
            display_size = size / GIBIBYTE_FACTOR;
            /* Translators: SI prefix and standard unit symbol, translate cautiously (or not at all) */
            unit = pgettext("byte-size-pow2", "GiB");
        } else {
            display_size = size / TEBIBYTE_FACTOR;
            /* Translators: SI prefix and standard unit symbol, translate cautiously (or not at all) */
            unit = pgettext("byte-size-pow2", "TiB");
        }

        let digits = if display_size < 10.0 { 1 } else { 0 };

        format!("{:.digits$} {}", display_size, unit)
    }

    fn pow10_size(&self, size: u64) -> String {
        let size = size as f64;

        let display_size;
        let unit;
        if size < MEGABYTE_FACTOR {
            display_size = size / KILOBYTE_FACTOR;
            /* Translators: SI prefix and standard unit symbol, translate cautiously (or not at all) */
            unit = pgettext("byte-size-pow10", "KB");
        } else if size < GIGABYTE_FACTOR {
            display_size = size / MEGABYTE_FACTOR;
            /* Translators: SI prefix and standard unit symbol, translate cautiously (or not at all) */
            unit = pgettext("byte-size-pow10", "MB");
        } else if size < TERABYTE_FACTOR {
            display_size = size / GIGABYTE_FACTOR;
            /* Translators: SI prefix and standard unit symbol, translate cautiously (or not at all) */
            unit = pgettext("byte-size-pow10", "GB");
        } else {
            display_size = size / TERABYTE_FACTOR;
            /* Translators: SI prefix and standard unit symbol, translate cautiously (or not at all) */
            unit = pgettext("byte-size-pow10", "TB");
        }

        let digits = if display_size < 10.0 { 1 } else { 0 };

        format!("{:.digits$} {}", display_size, unit)
    }

    /// Utility function to get a human-readable string that represents the given size.
    ///
    /// When `use_pow2` is set to true power-of-two units are used instead of power-of-ten
    /// units.
    /// Set `long_str` to true, to produce a long string.
    pub fn size_for_display(&self, size: u64, use_pow2: bool, long_str: bool) -> String {
        let pow_size = if use_pow2 {
            self.pow2_size(size)
        } else {
            self.pow10_size(size)
        };

        if !long_str {
            return pow_size;
        }

        if use_pow2 {
            // Translators: The first %s is the size in power-of-2 units, e.g. '64 KiB'
            // the second %s is the size as a number e.g. '65,536' (always > 1)
            pgettext_f(
                "byte-size-pow2",
                "{} ({} bytes)",
                [pow_size, size.to_string()],
            )
        } else {
            // Translators: The first %s is the size in power-of-10 units, e.g. '100 kB'
            // the second %s is the size as a number e.g. '100,000' (always > 1)
            pgettext_f(
                "byte-size-pow10",
                "{} ({} bytes)",
                [pow_size, size.to_string()],
            )
        }
    }

    /// Returns a human readable localized string for `usage`, `type` and `version`.
    pub fn id_for_display(&self, usage: &str, ty: &str, version: &str, long_str: bool) -> String {
        ID_TYPES
            .iter()
            .filter(|id| id.usage == usage && id.ty == ty)
            .find_map(|id| {
                if id.version.is_none() && version.is_empty() {
                    return Some(if long_str {
                        dpgettext("fs-type", id.long_name)
                    } else {
                        dpgettext("fs-type", id.short_name)
                    });
                } else if !version.is_empty()
                    && (id.version == Some(version) || id.version == Some("*"))
                {
                    return Some(if long_str {
                        dpgettext("fs-type", id.long_name).replace("%s", version)
                    } else {
                        dpgettext("fs-type", id.short_name).replace("%s", version)
                    });
                }
                None
            })
            .unwrap_or_else(|| {
                let id_type;
                if long_str {
                    if !version.is_empty() {
                        // Translators: Shown for unknown filesystem types.
                        // First %s is the raw filesystem type obtained from udev, second %s is version.
                        id_type = pgettext_f("fs-type", "Unknown ({} {})", [ty, version]);
                    } else if !ty.is_empty() {
                        // Translators: Shown for unknown filesystem types.
                        // First %s is the raw filesystem type obtained from udev.
                        id_type = pgettext_f("fs-type", "Unknown ({})", [ty]);
                    } else {
                        // Translators: Shown for unknown filesystem types.
                        id_type = pgettext("fs-type", "Unknown");
                    }
                } else if !ty.is_empty() {
                    id_type = ty.to_string();
                } else {
                    // Translators: Shown for unknown filesystem types.
                    id_type = pgettext("fs-type", "Unknown");
                }
                id_type
            })
    }

    /// Returns a human-readable, localized string of the media described by the given `media_compat`.
    ///
    /// If the media is unknown, [`Option::None`] is returned.
    pub fn media_compat_for_display(&self, media_compat: &[&str]) -> Option<String> {
        let mut optical_cd = false;
        let mut optical_dvd = false;
        let mut optical_bd = false;
        let mut optical_hddvd = false;
        let mut media_desc: String = media_compat
            .iter()
            .filter_map(|&media| match media {
                "flash_cf" => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "CompactFlash"))
                }
                "flash_ms" => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "MemoryStick"))
                }
                "flash_sm" => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "SmartMedia"))
                }
                "flash_sd" => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "SecureDigital"))
                }
                "flash_sdhc" => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "SD High Capacity"))
                }
                "floppy" => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "Floppy"))
                }
                "floppy_zip" => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "Zip"))
                }
                "floppy_jaz" => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "Jaz"))
                }
                val if val.starts_with("flash") => {
                    // Translators: This word is used to describe the media inserted into a device
                    Some(pgettext("media", "Flash"))
                }
                val => {
                    if val.starts_with("optical_cd") {
                        optical_cd = true;
                    } else if val.starts_with("optical_dvd") {
                        optical_dvd = true;
                    } else if val.starts_with("optical_bd") {
                        optical_bd = true;
                    } else if val.starts_with("optical_hddvd") {
                        optical_hddvd = true;
                    }
                    None
                }
            })
            //TODO: replace with intersperse
            .collect::<Vec<_>>()
            .join(",");

        let add_separator = |str: &mut String| {
            if !str.is_empty() {
                str.push('/');
            }
        };

        if optical_cd {
            add_separator(&mut media_desc);
            //Translators: This word is used to describe the optical disc type, it may appear
            // in a slash-separated list e.g. 'CD/DVD/Blu-Ray'
            media_desc.push_str(&pgettext("disc-type", "CD"));
        }
        if optical_dvd {
            add_separator(&mut media_desc);
            //Translators: This word is used to describe the optical disc type, it may appear
            // in a slash-separated list e.g. 'CD/DVD/Blu-Ray'
            media_desc.push_str(&pgettext("disc-type", "DVD"));
        }
        if optical_bd {
            add_separator(&mut media_desc);
            //Translators: This word is used to describe the optical disc type, it may appear
            // in a slash-separated list e.g. 'CD/DVD/Blu-Ray'
            media_desc.push_str(&pgettext("disc-type", "Blu-Ray"));
        }
        if optical_hddvd {
            add_separator(&mut media_desc);
            //Translators: This word is used to describe the optical disc type, it may appear
            // in a slash-separated list e.g. 'CD/DVD/Blu-Ray'
            media_desc.push_str(&pgettext("disc-type", "HDDVD"));
        }

        //return none, if the string is empty, to clearly indicate that the media is unknown
        //it is also closer to the C API
        if media_desc.is_empty() {
            None
        } else {
            Some(media_desc)
        }
    }

    /// Returns information about all known partition types for `partition_table_type` (e.g. `dos` or `gpt`) and `partition_table_subtype`.
    ///
    /// If `partition_table_subtype` is [`None`], it is equivalent to all known types.
    pub fn partition_type_infos(
        &self,
        partition_table_type: &str,
        partition_table_subtype: Option<&str>,
    ) -> Vec<&PartitionTypeInfo> {
        //TODO: use enum for table type?
        //TODO: C version uses a custom type, which appears to be the same as `PartitionTypeInfo`,
        //but without the name
        //https://github.com/storaged-project/udisks/blob/4f24c900383d3dc28022f62cab3eb434d19b6b82/udisks/udisksclient.c#L2604
        PARTITION_TYPES
            .iter()
            .filter(|pti| {
                pti.table_type == partition_table_type
                    && (partition_table_subtype.is_none()
                        || Some(pti.table_subtype) == partition_table_subtype)
            })
            .collect()
    }

    /// Returns information about all known subtypes for `partition_table_type` (e.g. `dos` or `gpt`) and `partition_table_subtype`.
    pub fn partition_table_subtypes(&self, partition_table_type: &str) -> Vec<&str> {
        partition_subtypes::PARTITION_TABLE_SUBTYPES
            .iter()
            .filter(|pt| pt.ty == partition_table_type)
            .map(|pt| pt.subtype)
            .collect()
    }

    /// Returns, if exists, the human-readable localized name of the [PartitionTypeInfo].
    pub fn partition_type_for_display(
        &self,
        partition_table_type: &str,
        partition_type: &str,
    ) -> Option<String> {
        partition_types::PARTITION_TYPES
            .iter()
            .find(|pt| pt.table_type == partition_table_type && pt.ty == partition_type)
            .map(|partition_type| dpgettext("part-type", partition_type.name))
    }

    /// Returns, if existing, the human-readable localized name of the [`PartitionTypeInfo`].
    ///
    /// It is similar to [`Client::partition_type_for_display`], but also accounts for the `partition_table_subtype`, if available.
    /// This can be useful for scenarios, where different subtypes are using the same partition
    /// type.
    pub fn partition_type_and_subtype_for_display(
        &self,
        partition_table_type: &str,
        partition_table_subtype: &str,
        partition_type: &str,
    ) -> Option<String> {
        PARTITION_TYPES
            .iter()
            .filter(|pt| pt.table_type == partition_table_type && pt.ty == partition_type)
            .filter(|pt| partition_table_subtype == pt.table_subtype)
            .map(|pt| dpgettext("part-type", pt.name))
            .next()
    }

    /// Returns, if exists, the human-readable localized string for `partition_table_type` (e.g.
    /// `dos` or `gpt`).
    pub fn partition_table_type_for_display(&self, partition_table_type: &str) -> Option<String> {
        //TODO: use enum
        [
            // Translators: name of partition table format
            ("dos", pgettext("dos", "Master Boot Record")),
            // Translators: name of partition table format
            ("gpt", pgettext("gpt", "GUID Partition Table")),
            // Translators: name of partition table format
            ("apm", pgettext("apm", "Apple Partition Map")),
        ]
        .iter()
        .find(|(ty, _)| ty == &partition_table_type)
        .map(|(_, name)| name.to_string())
    }

    /// Returns a human-readable localized description for `partition_table_type` (e.g. `dos` or `gpt`)
    /// and `partition_table_subtype` (e.g. `dos` or `gpt`).
    pub fn partition_table_subtype_for_display(
        &self,
        partition_table_type: &str,
        partition_table_subtype: &str,
    ) -> Option<String> {
        //TODO: C version docs for subtype and type are identical, bug?
        partition_subtypes::PARTITION_TABLE_SUBTYPES
            .iter()
            .find(|pt| pt.ty == partition_table_type && pt.subtype == partition_table_subtype)
            .map(|pt| dpgettext("partition-subtype", pt.name))
    }
}
