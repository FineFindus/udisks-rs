//! D-Bus interface for Linux Software RAID arrays.
//!
//! Objects implementing this interface represent [Linux Software RAID arrays](https://raid.wiki.kernel.org/index.php/Linux_Raid)
//! detected on the system. Both running and stopped arrays are represented.
//!
//! Block devices point to objects implementing this interface, see the
//! [`org.freedesktop.UDisks2.Block:MDRaid`](crate::block::BlockProxy::mdraid) and [`org.freedesktop.UDisks2.Block:MDRaidMember`](crate::block::BlockProxy::mdraid_member)
//! properties on the [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy) interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

use crate::{error, manager::RaidLevel};

/// Sync action to request for [`MDRaidProxy::request_sync_action`].
#[derive(Debug, serde::Serialize, zbus::zvariant::Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SyncAction {
    /// Verify data consistency without repair
    Check,
    /// Check and repair inconsistent data
    Repair,
    /// Cancel any ongoing sync operation
    Idle,
}

/// Information about an active device associated with a raid array.
///
/// Can be obtained from [`MDRaidProxy::active_devices`].
#[derive(
    Debug,
    zbus::zvariant::Type,
    zbus::zvariant::Value,
    zbus::zvariant::OwnedValue,
    serde::Deserialize,
)]
pub struct ActiveDevice {
    /// The object path for the underlying block device
    /// (guaranteed to implement the [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy) interface).
    pub object_path: OwnedObjectPath,
    /// Slot number the device currently fills (between `0` and [`MDRaidProxy::num_devices`]).
    ///
    /// `-1` if the device is not currently part of the array (i.e. `spare` or `faulty`).
    pub slot: i32,
    /// State of the device.
    pub state: Vec<DeviceState>,
    /// Ongoing count of read errors that have been detected on this device but have not caused the device to be evicted from the array.
    pub num_read_errors: u64,
    /// Reserved for future expansion (currently unused).
    pub expansion: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
}

/// State of the [`ActiveDevice`].
#[derive(
    Debug,
    serde::Deserialize,
    zbus::zvariant::Type,
    zbus::zvariant::Value,
    zbus::zvariant::OwnedValue,
)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeviceState {
    Faulty,
    InSync,
    WriteMostly,
    Blocked,
    Spare,
}

#[proxy(
    interface = "org.freedesktop.UDisks2.MDRaid",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/MDRaid"
)]
pub trait MDRaid {
    /// Adds `device` to the array.
    ///
    /// `device` must implement the [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy)
    /// interface.
    fn add_device(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Stops the RAID array and destroys all RAID metadata on member devices.
    ///
    /// If the option `tear-down` is set to `true`, then the RAID array block
    /// device and all its children will be cleaned up before stopping.
    /// This cleanup consists of removing entries from `/etc/fstab` and `/etc/crypttab`,
    /// and locking of encrypted block devices. Entries in `/etc/fstab` and
    /// `/etc/crypttab` that have been created with the 'track-parents' options
    /// to [`BlockProxy::add_configuration_item`](crate::block::BlockProxy::add_configuration_item)
    /// will be removed even if their block device is currently unavailable.
    fn delete(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Removes `device` from the array.
    ///
    /// For this to work `device` must already be associated
    /// with the array, e.g. referenced in the [`Self::active_devices`] property.
    ///
    /// If the `options` parameter contains the key `wipe` with the value `true`,
    /// all known filesystems will be erased from the `device` after removal.
    ///
    /// `device` must implement the [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy)
    /// interface.
    fn remove_device(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// This method call can be used to trigger and cancel data
    /// redundancy checks and repairs.
    ///
    /// # See Also
    /// [`Self::sync_action`]
    ///
    /// This method call is similar to writing to the
    /// `sync_actiona` sysfs file, see the
    /// [Documentation/admin-guide/md.rst](https://www.kernel.org/doc/Documentation/admin-guide/md.rst)
    /// file shipped with the kernel sources.
    fn request_sync_action(
        &self,
        sync_action: SyncAction,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets whether the array has a write-intent bitmap.
    ///
    /// Currently the `value` supports `none` and `internal` as possible values.
    fn set_bitmap_location(
        &self,
        //TODO: support using an enum
        value: &[u8],
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Starts the RAID array.
    ///
    /// If the `option` parameter contains the key `start-degraded` with the value `true`,
    /// the array will be started even if some members are missing.
    fn start(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Stops the RAID array.
    fn stop(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Vector of block devices that are currently associated with the array.
    ///
    /// It is empty if the array is not running.
    #[zbus(property)]
    fn active_devices(&self) -> error::Result<Vec<ActiveDevice>>;

    /// The location of a write-intent bitmap (empty if the array is not running), if any.
    ///
    /// If the RAID array does not support write-intent bitmaps (for example RAID-0 arrays),
    /// this is empty. This property corresponds to the `bitmap` sysfs file, see the
    /// [Documentation/admin-guide/md.rst](https://www.kernel.org/doc/Documentation/admin-guide/md.rst)
    #[zbus(property)]
    fn bitmap_location(&self) -> error::Result<Vec<u8>>;

    /// Configuration items belonging to the block devices of this array (recursively).
    ///
    /// This is also valid when the array is stopped and there are no actual block devices for it.
    /// It works via the 'track-parents' options of [`BlockProxy::add_configuration_item`](crate::block::BlockProxy::add_configuration_item),
    /// which see.
    #[zbus(property)]
    fn child_configuration(
        &self,
    ) -> error::Result<
        Vec<(
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// Chunk size (0 if the array is not running or not using striping).
    ///
    /// This property corresponds to the `chunk_size` sysfs file, see the
    /// [Documentation/admin-guide/md.rst](https://www.kernel.org/doc/Documentation/admin-guide/md.rst)
    /// file shipped with the kernel sources.
    #[zbus(property)]
    fn chunk_size(&self) -> error::Result<u64>;

    /// Number of devices by which the array is degraded (0 if not degraded or not running).
    ///
    /// This property corresponds to the `chunk_size` sysfs file, see the
    /// [Documentation/admin-guide/md.rst](https://www.kernel.org/doc/Documentation/admin-guide/md.rst)
    /// file shipped with the kernel sources.
    #[zbus(property)]
    fn degraded(&self) -> error::Result<u32>;

    /// RAID level of the array.
    #[zbus(property)]
    fn level(&self) -> error::Result<RaidLevel>;

    /// Name of the RAID array.
    #[zbus(property)]
    fn name(&self) -> error::Result<String>;

    /// Number of devices that are part of the array.
    #[zbus(property)]
    fn num_devices(&self) -> error::Result<u32>;

    /// Whether the array is currently running.
    ///
    /// It is an error to call Start on a running array, and Stop on
    /// a non-running array, for example.
    #[zbus(property)]
    fn running(&self) -> error::Result<bool>;

    /// Size of the array or 0 if unknown.
    ///
    /// This is the usable size, e.g. for a RAID-5 array backed by 4
    /// 1TB disks, this will be approximately 3 TB.
    #[zbus(property)]
    fn size(&self) -> error::Result<u64>;

    /// Current synchronization action being performed.
    ///
    /// Returns the current state of any ongoing sync operation, or empty
    /// if the array is not running or has no redundancy (e.g., RAID-0).
    ///
    /// # Returns
    /// Current sync action (e.g., `"check"`, `"repair"`, `"recover"`) or empty
    ///
    /// # See Also
    /// [`request_sync_action`](Self::request_sync_action) - Method to change this state
    #[zbus(property)]
    fn sync_action(&self) -> error::Result<String>;

    /// Fraction of sync operation completed.
    ///
    /// Progress of any ongoing synchronization operation, always between
    /// 0.0 and 1.0. Returns 0.0 if no operation is in progress.
    ///
    /// # Returns
    /// Completion fraction (0.0 = not started, 1.0 = complete)
    #[zbus(property)]
    fn sync_completed(&self) -> error::Result<f64>;

    /// Rate (or speed) at which the sync operation takes
    /// place. It is averaged over the last 30 seconds and measured
    /// in bytes per second.
    ///
    /// If the rate is unknown or no operation is in progress, the
    /// value of this property is 0.
    ///
    /// This property corresponds to the `sync_speed` sysfs file, see the
    /// [Documentation/admin-guide/md.rst](https://www.kernel.org/doc/Documentation/admin-guide/md.rst)
    /// file shipped with the kernel sources.
    #[zbus(property)]
    fn sync_rate(&self) -> error::Result<u64>;

    /// Estimated number of micro-seconds until the operation is
    /// finished
    ///
    /// If the amount of remaining time is unknown or no operation is
    /// in progress, the value of this property is 0.
    ///
    /// This property is based on the value of the`sync_speed` sysfs file, see the
    /// [Documentation/admin-guide/md.rst](https://www.kernel.org/doc/Documentation/admin-guide/md.rst)
    /// file shipped with the kernel sources.
    #[zbus(property)]
    fn sync_remaining_time(&self) -> error::Result<u64>;

    /// UUID of the RAID array.
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> error::Result<String>;
}
