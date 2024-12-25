//! Interface to represent a block device.
//!
//! This interface should not to be confused with the
//! [`org.freedesktop.UDisks2.Drive`](crate::drive) that is used to represent drives.
//! For example, the [`org.freedesktop.UDisks2.Block`](crate::block) interface
//! is also used for block devices that do not correspond to drives at all
//! (e.g. [Loop Devices](https://en.wikipedia.org/wiki/Loop_device)).

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.Block",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Block"
)]
pub trait Block {
    /// Adds a new configuration item.
    ///
    /// See [`Self::configuration`] for details.
    /// Some fields can be omitted and will then receive default values. This is useful when passing configuration items to Format,
    /// for example, because the proper values are not known before the formatting is done.
    ///
    /// * If `fsname` is omitted in a `fstab` entry, or `device` is omitted in a `crypttab` entry,
    /// it defaults to `UUID=...` when the block device has a filesystem UUID,
    /// or to the name of the device in the filesystem.
    ///
    /// * If `name` is omitted in a `crypttab` entry, it defaults to `luks-<UUID>`.
    ///
    /// * If `passphrase-path` is omitted, it defaults to `/etc/luks-keys/<NAME>`.
    ///
    /// * If `track-parents` is set to true in item,
    /// then the `opts` and `options` fields will be augmented with `x-parent` elements,
    /// as appropriate. This will make item appear in the ChildConfiguration properties,
    /// and will allow the `tear-down` option of Format, DeletePartition,
    /// and other methods to remove this item reliably.
    fn add_configuration_item(
        &self,
        //TODO: use struct
        item: &(
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    //TODO: use enum for type
    /// Formats the device with a file system, partition table or other well-known content.
    ///
    /// Known values for type includes `empty` (to just zero out areas of the device known to host file system signatures) and `swap` (Linux swap space)
    /// and most file systems supported by the mkfs(8) program through its `-t` option.
    ///
    /// Known partition table formats includes `dos` and `gpt`.
    ///
    /// If `type` supports it, you can specify a label with the `label` option in the `options` parameter; however,
    /// note that this may not be supported on all file systems and, if supported, the maximum allowed length may vary.
    /// Similarly, you can specify filesystem UUID with the `uuid` option in the options parameter given that the particular filesystem type supports this feature.
    /// The format of the UUID string should match the [`Self::uuid`] property.
    ///
    /// If the file system in question supports owners and the option `take-ownership` is set to `true` then the root directory of the created file system will be owned by the caller of this method.
    ///
    /// If the option `encrypt.passphrase` is given then a LUKS device is created with the given passphrase and the file system is created on the unlocked `device`.
    /// The unlocked device will be left open. This parameter can be used to supply the binary contents of an arbitrary keyfile by passing a value of type `ay`.
    /// Option `encrypt.type` can be used to specify encryption "technology" that will be used.
    /// Currently only “luks1” and “luks2” are supported.
    /// Following additional options for LUKS key derivation function can be used:
    /// - `encrypt.pbkdf`: key derivation function, one of "pbkdf2", "argon2i", "argon2id"
    /// - `encrypt.iterations`: number of iterations for PBKDF
    /// - `encrypt.memory`: memory cost in KiB for Argon2
    /// - `encrypt.time`: time cost for PBKDF in ms
    /// - `encrypt.threads`: parallel cost for PBKDF (number of threads, up to 4)
    ///
    /// If the option `erase` is used then the underlying device will be erased.
    /// Valid values include “zero” to write zeroes over the entire device before formatting,
    /// “ata-secure-erase” to perform a secure erase or “ata-secure-erase-enhanced” to perform an enhanced secure erase.
    ///
    /// If the option `update-partition-type` is set to `true` and the object in question is a partition, then its type (cf. the "Type" property)
    /// will be set to the natural partition type matching type, if any.
    /// For example, if formatting a GPT partition with a FAT filesystem, the “Microsoft Basic Data” partition type will be chosen;
    /// similar, if formatting a DOS partition with a Ext4 filesystem then partition type 0x83 is chosen.
    ///
    /// If the option `no-block` is set to `true` then the method returns just before the actual formatting takes place
    /// but after authorization and other checks are done. This is useful for applications that want to format several devices in parallel.
    ///
    /// If the option `dry-run-first` is set to `true` then a dry run of the formatting command is performed first,
    /// if UDisks knows how to do that. The idea is that this allows a deeper check of the parameters even when `no-block` is `true`.
    /// Note that the block device has already been modified (wiped) when the dry run check is called.
    ///
    /// If the option `mkfs-args` is set then the caller can provide a list of additional command-line arguments that will be passed to the mkfs program.
    /// The arguments specified in this way are not validated by UDisks, and the user is responsible for making sure that the available mkfs program
    /// for that filesystem supports them and that they work for the intended purpose.
    /// Note that UDisks can also pass its own additional arguments to mkfs, and they may vary between releases, with no guarantees of stability in this regard.
    /// The position in which the user-provided arguments are appended to the final mkfs command line is also not defined.
    /// Because of all this, `mkfs-args` should only be used as a last resort when no other dedicated option is available.
    ///
    /// If the option `no-discard` is set to `true` then Udisks tells the formatting utility not to issue `BLKDISCARD` ioctls.
    ///
    /// If the option `config-items` is set, it should be an array of configuration items suitable for [`Self::add_configuration_item`].
    /// They will all be added after the formatting is done.
    ///
    /// If the option `tear-down` is set to `true`, then the block device and all its children will be cleaned up before formatting.
    /// This cleanup consists of removing entries from `/etc/fstab` and `/etc/crypttab`, and locking of encrypted block devices.
    /// Entries in `/etc/fstab` and `/etc/crypttab` that have been created with the 'track-parents' options to [`Self::add_configuration_item`]
    /// will be removed even if their block device is currently unavailable.
    ///
    /// Note that if the operation fails the block device may be left in an inconsistent state.
    /// In cases of removing partition table and failed operation it's advised to validate the device e.g.
    /// by re-reading the partition table or force-wiping it before performing further operations.
    ///
    /// In case the `tear-down` option is not set and the block device being formatted is partitioned and contains mounted filesystem
    /// or an active layered structure inside then the Format operation may not fail, yet could still overwrite nested foreign data regions.
    /// It is the caller responsibility to ensure the device is ready for destructive operations. This may be subject to further restrictions in the future.
    fn format(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Returns the same value as [`Self::configuration`], but without secret information filtered out.
    fn get_secret_configuration(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<
        Vec<(
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// Returns a file desciptor to the device.
    ///
    /// Set option `flags` for additional flags. See man 2 open for list of supported flags.
    /// `O_RDONLY`, `O_WRONLY` and `O_RDWR` are not allowed, use `mode` instead.
    ///
    /// `mode` specifies the mode that the file can be opened, can be `r`(read-only),
    /// `w`(write-only) and `rw`(read-write).
    fn open_device(
        &self,
        //TODO: use enum/struct
        mode: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedFd>;

    /// Returns a read-only file descriptor for the device intended for a byte-by-byte imaging of the device.
    /// This can only be done if the device is not already in use.
    #[deprecated = "Use OpenDevice with O_EXCL and O_CLOEXEC flags instead."]
    fn open_for_backup(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedFd>;

    /// Returns a file descriptor for the device that is suitable to be used for benchmarking the device
    /// (transfer rate, access time etc.).
    ///
    /// Note that the file descriptor may be opened with the O_DIRECT and O_SYNC flags so care must be taken to
    /// only perform block-aligned I/O.
    ///
    /// If the `writable` in options is `true` then the returned file descriptor will be writable.
    /// This only works if the device is not already in use.
    #[deprecated = "Use OpenDevice with O_DIRECT, O_SYNC and O_CLOEXEC flags instead."]
    fn open_for_benchmark(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedFd>;

    /// Returns a writable file descriptor for the device intended for a byte-by-byte restore
    /// of a disk image onto the device.
    ///
    /// This can only be done if the device is not already in use.
    #[deprecated = "Use OpenDevice with O_EXCL and O_CLOEXEC flags instead."]
    fn open_for_restore(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedFd>;

    /// Removes an existing configuration item.
    ///
    /// See the [`Self::configuration`] property for details about valid configuration items.
    fn remove_configuration_item(
        &self,
        item: &(
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Request that the kernel and core OS rescans the contents of the device and update their state to reflect this
    /// (including things such as the `/dev/disk/` hierarchy of symlinks).
    ///
    /// This includes requesting that the kernel re-reads the partition table, if appropriate.
    /// This is usually not needed since the OS automatically does this when the last process
    /// with a writable file descriptor for the device closes it.
    fn rescan(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Removes a configuration item and adds a new one.
    ///
    /// This is equivalent to calling [`Self::remove_configuration_item`] followed by [`Self::add_configuration_item`]
    /// with the change that only one PolicyKit check is made and that `new_item` can be validated against `old_item`.
    ///
    /// See the [`Self::configuration`] property for details about valid configuration items.
    fn update_configuration_item(
        &self,
        old_item: &(
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        new_item: &(
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// The configuration for the device.
    ///
    /// This is an array of tuples `(type, details)` where type is a string identifying the configuration source
    /// (e.g. `fstab`) and details contains the actual configuration data.
    ///
    /// Use the [`Self::add_configuration_item`], [`Self::remove_configuration_item`] and [`Self::update_configuration_item`]
    /// methods to add, remove and update configuration items.
    ///
    /// Use [`Self::get_secret_configuration`] to get the secrets (e.g. `passphrases`) that may be part of the configuration
    /// but isn't exported in this property for security reasons.
    ///
    /// For entries of type `fstab`, it means that the block device is referenced in the system-wide `/etc/fstab` file.
    /// Known configuration items for type `fstab` are:
    //TODO: link to zbus types
    /// - `fsname` (type `ay`): The special device
    /// - `dir` (type `ay`): The mount point
    /// - `type` (type `ay`): The filesystem point
    /// - `opts` (type `ay`): Options
    /// - `freq` (type `i`): Dump frequency in days
    /// - `passno` (type `i`): Pass number of parallel `fsck`
    ///
    /// For entries of type `crypttab`, it means that the block device is referenced in the system-wide `/etc/crypttab` file.
    /// Known configuration items for type crypttab are:
    /// - `name` (type `ay`): The name to set the device up as
    /// - `device` (type `ay`): The special device
    /// - `passphrase-path` (type `ay`): Either empty to specify that no password is set, otherwise a path to a file containing the encryption password.
    ///     This may also point to a special device file in `/dev` such as `/dev/random`.
    /// - `passphrase-contents` (type `ay`): The contents of the file containing the encryption password, if applicable. This is only available via the [`Self::get_secret_configuration`] method.
    /// - `options` (type `ay`): Options
    ///
    /// For security reasons, when creating a new `crypttab` entry (via the [`Self::add_configuration_item`] method), then the `passphrase-path`
    /// must reference an unexisting file in the `/etc/luks-keys` directory.
    #[zbus(property)]
    fn configuration(
        &self,
    ) -> error::Result<
        Vec<(
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// The [org.freedesktop.UDisks2.Block](crate::block) object that is backing the device
    /// or `/` if unknown or if the block device is not the cleartext device for an encrypted device.
    #[zbus(property)]
    fn crypto_backing_device(&self) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    ///The special device file for the block device e.g. `/dev/sda2`.
    #[zbus(property)]
    fn device(&self) -> error::Result<Vec<u8>>;

    /// The `dev_t` of the block device.
    #[zbus(property)]
    fn device_number(&self) -> error::Result<u64>;

    /// The [`org.freedesktop.UDisks2.Drive `](crate::drive) object that the block device belongs to.
    /// Returns '/' if no such object exists.
    #[zbus(property)]
    fn drive(&self) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// Whether the device should be automatically started (e.g. mounted, unlocked etc.).
    ///
    /// See [udisks(8)](https://storaged.org/doc/udisks2-api/latest/udisks.8.html) for how to influence the value of this property.
    #[zbus(property)]
    fn hint_auto(&self) -> error::Result<bool>;

    /// If not blank, the icon name to use when presenting the device.
    ///
    /// The name must adhere to the [freedesktop.org icon theme specification](http://www.freedesktop.org/wiki/Specifications/icon-theme-spec).
    ///
    /// See [udisks(8)](https://storaged.org/doc/udisks2-api/latest/udisks.8.html) for how to influence the value of this property.
    #[zbus(property)]
    fn hint_icon_name(&self) -> error::Result<String>;

    /// Whether the device should be hidden from users.
    ///
    /// See [udisks(8)](https://storaged.org/doc/udisks2-api/latest/udisks.8.html) for how to influence the value of this property.
    #[zbus(property)]
    fn hint_ignore(&self) -> error::Result<bool>;

    ///If not blank, the name to use when presenting the device.
    ///
    /// See [udisks(8)](https://storaged.org/doc/udisks2-api/latest/udisks.8.html) for how to influence the value of this property.
    //TODO: is it possible to use NONE instead of a blank string
    #[zbus(property)]
    fn hint_name(&self) -> error::Result<String>;

    /// Whether the device is normally expected to be partitionable.
    /// Devices for which this is not the case include floppy drives, optical drives and LVM logical volumes.
    #[zbus(property)]
    fn hint_partitionable(&self) -> error::Result<bool>;

    /// If not blank, the icon name to use when presenting the device using a symbolic icon.
    ///
    /// The name must adhere to the [freedesktop.org icon theme specification](http://www.freedesktop.org/wiki/Specifications/icon-theme-spec).
    ///
    /// See [udisks(8)](https://storaged.org/doc/udisks2-api/latest/udisks.8.html) for how to influence the value of this property.
    #[zbus(property)]
    fn hint_symbolic_icon_name(&self) -> error::Result<String>;

    /// Whether the device is considered a system device.
    /// System devices are devices that require additional permissions to access.
    ///
    /// See [udisks(8)](https://storaged.org/doc/udisks2-api/latest/udisks.8.html) for how to influence the value of this property.
    #[zbus(property)]
    fn hint_system(&self) -> error::Result<bool>;

    /// A unique and persistent identifier for the device.
    /// Empty if no such identifier is available.
    ///
    /// For devices with fixed media, this identifier is derived from vital product data / UUIDs / serial numbers
    /// of the drive or construct (e.g. LVM or MD-RAID) the block device is part of.
    /// For devices with removable media, this identifier is derived from the medium currently inserted.
    /// This identifier is guaranteed to not include the slash character '/' (U+002F SOLIDUS) which means it
    /// can be used as a filename.
    ///
    /// # Examples
    ///
    /// * `by-id-ata-INTEL_SSDSA2MH080G1GC_CVEM842101HD080DGN`
    /// * `by-id-ata-ST1000LM024_HN-M101MBB_S2TBJA0C230233-part3`
    /// * `by-id-usb-Kingston_DataTraveler_2.0_0013729940C4F9A166250D3E-0:0`
    /// * `by-id-dm-name-luks-6d81fe85-26b1-4f8b-b834-405454c1cd46`
    /// * `by-id-dm-name-vg_thinkpad-lv_swap`
    /// * `by-label-HARRY_POTTER_SORCERERS_STONE-`
    /// * `by-uuid-D22D-08B8`
    #[zbus(property)]
    fn id(&self) -> error::Result<String>;

    /// The label of the filesystem or other structured data on the block device.
    ///
    /// Returns an empty string if there is no label or the label is unknown.
    #[zbus(property)]
    fn id_label(&self) -> error::Result<String>;

    /// This property contains more information about the result of probing the block device.
    ///
    /// The value depends of the value of [`Self::id_usage`]:
    ///
    /// * `filesystem`: The mountable file system that was detected (e.g. `vfat`).
    /// * `crypto`: Encrypted data. Known values include `crypto_LUKS`.
    /// * `raid`: RAID or similar. Known values include `LVM2_member` (for LVM2 components),
    ///    `linux_raid_member` (for MD-RAID components.)
    /// * `other`: Something else. Known values include `swap` (for swap space),
    ///    `suspend` (data used when resuming from suspend-to-disk).
    ///
    /// See the note for the "IdUsage" property about usage.
    //TODO: what?
    #[zbus(property)]
    fn id_type(&self) -> error::Result<String>;

    /// The [UUID](https://en.wikipedia.org/wiki/UUID) of the filesystem or other structured data on the block device.
    /// Do not make any assumptions about the UUID as its format depends on what kind of data is on the device.
    ///
    /// Returns an empty string if there is no label or the label is unknown.
    #[zbus(property, name = "IdUUID")]
    fn id_uuid(&self) -> error::Result<String>;

    /// A result of probing for signatures on the block device. Known values include:
    ///
    /// * `filesystem`: Used for mountable filesystems
    /// * `crypto`: Used for e.g. LUKS devices
    /// * `raid`: Used for e.g. RAID members and LVM PVs
    /// * `other`: Something else was detected.
    ///
    /// If blank, no known signature was detected. This doesn't necessarily mean the device contains no
    /// structured data; it only means that no signature known to the probing code was detected.
    ///
    /// Applications should not rely on the value, or the value of [`Self::id_type`]
    /// - instead, applications should check for whether the object in question implements interfaces
    /// such as e.g. [`org.freedesktop.UDisks2.Filesystem`](crate::filesystem),
    /// [`org.freedesktop.UDisks2.Swapspace`](crate::swapspace) or [`org.freedesktop.UDisks2.Encrypted`](crate::encrypted).
    //TODO: use enum
    #[zbus(property)]
    fn id_usage(&self) -> error::Result<String>;

    /// The version of the filesystem or other structured data on the block device.
    /// Do not make any assumptions about the format.
    ///
    /// Returns an empty string if there is no version or the version is unknown.
    #[zbus(property)]
    fn id_version(&self) -> error::Result<String>;

    /// If the block device is a running MD-RAID array,
    /// this is set to the [`org.freedesktop.UDisks2.MDRaid`](crate::mdraid) object that it correspond to.
    /// Returns '/' if no such object exists.
    #[zbus(property, name = "MDRaid")]
    fn mdraid(&self) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// If the block device is a member of a MD-RAID array,
    /// this is set to the [`org.freedesktop.UDisks2.MDRaid`](crate::mdraid) object that it correspond to.
    /// Returns '/' if no such object exists.
    #[zbus(property, name = "MDRaidMember")]
    fn mdraid_member(&self) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    //TODO: a lot of functions return Strings as c type strings (i.e. vec of u8 with \0 bytes)
    //they should be updated to return rust strings
    /// The special device file to present in the UI instead of the value of the [`Self::device`] property.
    ///
    /// For example this could be e.g. `/dev/mapper/mpathk` for a multipath device with special
    /// device file `/dev/dm-9`.
    #[zbus(property)]
    fn preferred_device(&self) -> error::Result<Vec<u8>>;

    /// If `true`, the device can not be written to, only read from.
    #[zbus(property)]
    fn read_only(&self) -> error::Result<bool>;

    /// The size of the block device.
    #[zbus(property)]
    fn size(&self) -> error::Result<u64>;

    /// Known symlinks in `/dev` that points to the device file in the "Device" property.
    ///
    /// For example, this array could include symlinks such as `/dev/disk/by-id/ata-INTEL_SSDSA2MH080G1GC_CVEM842101HD080DGN`
    /// and `/dev/disk/by-id/wwn-0x5001517387d61905`.
    #[zbus(property)]
    fn symlinks(&self) -> error::Result<Vec<Vec<u8>>>;

    /// List of userspace mount options.
    #[zbus(property)]
    fn userspace_mount_options(&self) -> error::Result<Vec<String>>;
}
