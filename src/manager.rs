//! Interface for top-level manager singleton object
//! located at the object path `/org/freedesktop/UDisks2/Manager`.

use enumflags2::BitFlags;
use zbus::{proxy, zvariant::Type};

use crate::error;

/// Mode flags indicating if growing and/or shriking resize is available if mounted/unmounted.
///
/// The mode corresponds to bitwise-OR combined BDFSResizeFlags of the libblockdev FS plugin.
///
/// Can be obtained from [`ManagerProxy::can_resize`].
#[derive(Debug, Clone, Copy, Type)]
#[enumflags2::bitflags]
#[repr(u64)]
#[non_exhaustive]
pub enum ResizeFlags {
    /// Shrinking resize allowed when unmounted
    BdFsOfflineShrink = 2,
    /// Growing resize allowed when unmounted
    BdFsOfflineGrow = 4,
    /// Shrinking resize allowed when mounted
    BdFsOnlineShrink = 8,
    /// Growing resize allowed when mounted
    BdFsOnlineGrow = 16,
}

/// Raid Levels
///
/// Used for setting up a raid devices using [`ManagerProxy::mdraid_create`]
#[derive(Debug, serde::Serialize, zbus::zvariant::Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
pub enum RaidLevel {
    Raid0,
    Raid1,
    Raid4,
    Raid5,
    Raid6,
    Raid10,
}

#[proxy(
    interface = "org.freedesktop.UDisks2.Manager",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Manager"
)]
pub trait Manager {
    /// Tests for availability to check the given filesystem.
    ///
    /// Returns whether checking is available and the required binary name if missing
    /// (i.e. no error and returns FALSE).
    ///
    /// # Errors
    ///
    /// Returns an error for unsupported filesystems or filesystems that do not
    /// support consistency checking.
    fn can_check(&self, type_: &str) -> error::Result<(bool, String)>;

    /// Tests for availability to create the given filesystem.
    ///
    /// See the [`Self::supported_filesystems`] property for a list
    /// of known types.
    ///
    /// Returns whether formatting is available and the required binary name if missing
    /// (i.e. no error and returns FALSE).
    ///
    /// # Errors
    ///
    /// Returns an error for unknown or unsupported filesystem types.
    fn can_format(&self, type_: &str) -> error::Result<(bool, String)>;

    /// Tests for availability to repair the given filesystem.
    ///
    /// Returns whether checking is available and the required binary name if missing
    /// (i.e. no error and returns FALSE).
    ///
    /// # Errors
    ///
    /// Returns an error for unsupported filesystems or filesystems which do not
    /// support repairing.
    fn can_repair(&self, type_: &str) -> error::Result<(bool, String)>;

    /// Tests availability to resize the given filesystem.
    ///
    /// The mode flags indicate if growing and/or shrinking resize is available
    /// when the filesystem is mounted or unmounted.
    ///
    /// Available since version 2.7.2.
    ///
    /// Returns whether resizing is available, the flags allowed for resizing (i.e. growing/shrinking support for online/offline)
    /// and the required binary name if missing (i.e. no error and returns FALSE).
    ///
    ///
    /// # Errors
    ///
    /// Returns an error for unknown filesystems or filesystems that do not
    /// support resizing.
    fn can_resize(&self, type_: &str) -> error::Result<(bool, BitFlags<ResizeFlags>, String)>;

    /// Loads and activates a single module by name.
    ///
    /// In case the module is already active, no reinitialization is performed and this
    /// call has no effect. Clients should call this method before using any
    /// particular module API. This action causes all objects to receive an `add`
    /// uevent, allowing the module to attach extra interfaces.
    ///
    /// Modules cannot be deactivated at the moment.
    ///
    /// Available since version 2.9.0.
    fn enable_module(&self, name: &str, enable: bool) -> error::Result<()>;

    /// Loads and activates modules.
    ///
    /// Modules that have already been loaded are not reinitialized on subsequent calls
    /// to this method and are skipped. In case any new module is getting activated by
    /// this method call a `add` uevent is triggered on all exported objects.
    /// This takes in account an optional explicit list of modules to load as specified
    /// in the `etc/udisks2/udisks2.conf` config file. If unspecified all available
    /// modules will be loaded.
    ///
    /// Modules cannot be deactivated at the moment. This method call never fails even
    /// if no module has been activated and by nature it cannot report any particular
    /// module initialization failures. Clients have no way of finding that a
    /// particular module is available.
    #[deprecated(note = "Use EnableModule instead")]
    fn enable_modules(&self, enable: bool) -> error::Result<()>;

    /// Gets a list of all block devices.
    ///
    /// Array of object paths for objects implementing the `org.freedesktop.UDisks2.Block` interface
    fn get_block_devices(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Creates a block device for the file represented by the given file description.
    ///
    /// Additionally, `offset` [`u64`], `size` [`u64`], `read-only` [`bool`], `no-part-scan` [`bool`] and `sector-size` [`u64`]
    /// can be set via `options`.
    /// Returns an object path to the object implementing the [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy) interface.
    fn loop_setup(
        &self,
        fd: zbus::zvariant::Fd<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// Creates a new RAID array on the block devices specified by
    /// the given blocks.
    ///
    /// Each element in this array must be an object path to
    /// an object implementing the [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy)
    /// interface.
    ///
    /// Before the array is created, all devices in `blocks` are
    /// erased. Once created (but before the method returns), the RAID
    /// array will be erased.
    ///
    /// The `bitmap` option specifies the write-intent bitmap type, currently
    /// only 'none' and 'internal' values are supported. When this option
    /// is not present, it is up to `mdadm` to decide
    /// whether to create an internal bitmap (typically for arrays larger
    /// than 100 GB) or not.
    ///
    /// The `version` option specifies the MD metadata version, for example
    /// '0.90'. If not specified the default medata version specified by
    /// `mdadm` is used. (since 2.11)
    #[zbus(name = "MDRaidCreate")]
    fn mdraid_create(
        &self,
        blocks: &[zbus::zvariant::ObjectPath<'_>],
        level: RaidLevel,
        name: &str,
        chunk: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// Get device(s) matching specification given in `devspec`.
    ///
    /// Currently supported keys for `devspec` include
    /// * `path` (type `String`) - Device path (e.g., "/dev/sda") including symlinks
    /// * `label` (type `String`) - Filesystem label
    /// * `uuid` (type `String`) - Filesystem UUID
    /// * `partuuid` (type `String`) - Partition UUID
    /// * `partlabel` (type `String`) - Partition name
    ///
    /// Available since version 2.7.3
    fn resolve_device(
        &self,
        //TODO: use a struct for the options
        devspec: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Default block encryption type.
    #[zbus(property)]
    fn default_encryption_type(&self) -> error::Result<String>;

    /// List of block encryption types supported by UDisks2.
    #[zbus(property)]
    fn supported_encryption_types(&self) -> error::Result<Vec<String>>;

    /// List of filesystems supported by UDisks2.
    ///
    /// For each such filesystem, UDisks2 supports filesystem creation ("mkfs")
    /// and changing labels.
    ///
    /// While UDisks2 can mount essentially any filesystem,
    /// only the listed types are fully supported for block operations.
    #[zbus(property)]
    fn supported_filesystems(&self) -> error::Result<Vec<String>>;

    /// Version the UDisks2 daemon currently running.
    #[zbus(property)]
    fn version(&self) -> error::Result<String>;
}
