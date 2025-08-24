//! This interface is used for [`org.freedesktop.UDisks2.Block`](crate::block) devices that contain
//! a mountable filesystem. It provides methods for mounting, unmounting, checking,
//! repairing, and managing filesystem properties.

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.Filesystem",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Filesystem"
)]
pub trait Filesystem {
    /// Check the filesystem for consistency without making any modifications or repairs, returning
    /// `true` if the filesystem is undamaged.
    ///
    /// Available since version 2.7.2.
    ///
    /// # Errors
    ///
    /// Mounted or unsupported filesystems will result in an error.
    fn check(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<bool>;

    /// Mount the filesystem.
    ///
    /// The directory the filesystem will be mounted in is determined by looking at data
    /// related to the device or filesystem (such as the filesystem UUID and label) and
    /// will be created automatically except if the device the filesystem resides on
    /// is referenced in the `/etc/fstab` file. In either case, the directory the
    /// filesystem is mounted in is returned on success - it is usually a sub-directory
    /// of `/run/media/$USER` but note that any directory may be returned.
    ///
    /// The filesystem type to use can be overridden with the `fstype` option and mount
    /// options (a comma-separated string) can be given in the `options` option. Note that
    /// both the mount options and filesystem types are validated against a (small) whitelist
    /// to avoid unexpected privilege escalation. The filesystem type is by default determined
    /// by the [`BlockProxy::id_type`](crate::block::BlockProxy::id_type) property.
    /// The `fstype` option doesn't typically need to be specified, primarily intended as an override
    /// in corner cases.
    ///
    /// If the `as-user` option is set, the filesystem is mounted on behalf of the specified
    /// user instead of the calling one. This has usually an effect on the returned mount path
    /// and it also allows that user to unmount the filesystem later. This option expects a
    /// user name, not a UID.
    ///
    /// If the device in question is referenced in the `/etc/fstab` file, the __mount__ command
    /// is called directly (as root) and the given options or filesystem type given in options
    /// are ignored.
    ///
    /// If `x-udisks-auth` is specified as an option for the device in the `/etc/fstab` file,
    /// then the __mount__ command is run as the calling user, without performing any authorization
    /// check mentioned above. If this fails because of insufficient permissions, an authorization
    /// check is performed (which typically results in the user having to authenticate as an
    /// administrator). If authorized, the __mount__ command is then run as root.
    ///
    /// The filesystem should be unmounted using the [`Self::unmount`] method.
    ///
    /// If the device is removed without being unmounted (e.g. the user yanking the device or
    /// pulling the media out) or unmounted in a way that bypasses the [`Self::unmount`] method
    /// (e.g. unmounted by the super-user by using the `umount` command directly), the device
    /// will be unmounted (if needed) and/or the mount point will be cleaned up.
    fn mount(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<String>;

    /// Tries to repair the filesystem, returning whether the repair was successful.
    ///
    /// Available since version 2.7.2.
    ///
    /// # Errors
    ///
    /// Mounted or unsupported filesystems will result in an error.
    fn repair(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<bool>;

    /// Resizes the filesystem to the specified size.
    ///
    /// Shrinking operations need to move data which causes this action to be slow.
    /// The filesystem-resize job for the object might expose progress information.
    ///
    /// Available since version 2.7.2.
    fn resize(
        &self,
        size: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets the filesystem label.
    fn set_label(
        &self,
        label: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets the filesystem UUID.
    ///
    /// Available since version 2.10.0.
    #[zbus(name = "SetUUID")]
    fn set_uuid(
        &self,
        uuid: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Changes ownership of the filesystem to the UID and GID of the calling user.
    ///
    /// Use `recursive` to recursively take ownership.
    ///
    /// # Errors
    ///
    /// Filesystems that don't support ownership will result in an error.
    ///
    /// Available since version 2.7.2.
    fn take_ownership(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Unmount a mounted device.
    ///
    /// If the device was mounted by the calling user via the [`Self::mount()`] method, the
    /// filesystem is unmounted without authorization checks. Otherwise, an authorization check is performed
    /// (which typically results in the user having to authenticate as an administrator).
    /// If authorized, the filesystem is unmounted.
    ///
    /// If the mountpoint was previously created by udisks it is guaranteed it will be removed
    /// upon returning from this method call.
    ///
    /// # Errors
    ///
    /// If the filesystem is busy, this operation fails with
    /// [`error::Error::DeviceBusy`] unless the `force` option is used.
    fn unmount(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// An array of filesystems paths for where the filesystem on
    /// the device is mounted. If the device is not mounted, this
    /// array is empty.
    #[zbus(property)]
    fn mount_points(&self) -> error::Result<Vec<Vec<u8>>>;

    ///  Size of the filesystem.
    ///  This is the amount of bytes used on the block device representing an outer filesystem
    ///  boundary. If this is smaller than [`BlockProxy::size`](crate::block::BlockProxy::size)
    ///  then the filesystem can be made larger with [`Self::resize`].
    ///
    ///  If the size is unknown, the property is zero. Currently limited
    ///  to xfs and ext filesystems only.
    ///
    ///  Please note that reading value of this property typically causes
    ///  some I/O to read the filesystem superblock. Unlike the rest
    ///  of the properties this one is set to be retrieved on-demand
    ///  and is not proactively cached by the daemon.
    #[zbus(property)]
    fn size(&self) -> error::Result<u64>;
}
