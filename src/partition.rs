//! Block device representing a partition.

use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use zbus::{proxy, zvariant::Type};

use crate::error;

/// Flags describing the partition.
#[bitflags]
#[repr(u64)]
#[derive(Type, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum PartitionFlags {
    /// The partition is marked as a system partition.
    ///
    /// Known flag for `gpt` partitions.
    SystemPartition = 1 << 0,
    /// The partition is marked as a Legacy BIOS Bootable partition.
    ///
    /// Known flag for `gpt` partitions.
    LegacyBIOSBootable = 1 << 2,
    /// The partition is marked as bootable.
    ///
    /// Known flag for `dos` partitions.
    Bootable = 0x80,
    /// The partition is marked as read-only.
    ///
    /// Known flag for `gpt` partitions.
    ReadOnly = 1 << 60,
    /// The partition is marked as hidden.
    ///
    /// Known flag for `gpt` partitions.
    Hidden = 1 << 62,
    /// The partition is marked as Do not automount.
    ///
    /// Known flag for `gpt` partitions.
    NoAutoMount = 1 << 63,
}

/// Generated code for the [`org.freedesktop.UDisks2.Partition`](https://storaged.org/doc/udisks2-api/latest/gdbus-org.freedesktop.UDisks2.Partition.html) D-Bus interface.
#[proxy(
    interface = "org.freedesktop.UDisks2.Partition",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Partition"
)]
trait Partition {
    /// Deletes the partition.
    ///
    /// If the option `tear-down` is set to `true`, then the block device and all its children will be cleaned up before formatting.
    /// This cleanup consists of removing entries from `/etc/fstab` and `/etc/crypttab`, and locking of encrypted block devices.
    /// Entries in `/etc/fstab` and `/etc/crypttab` that have been created with the 'track-parents' options to AddConfigurationItem
    /// will be removed even if their block device is currently unavailable.
    fn delete(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Resizes the partition.
    ///
    /// The partition will not change its position but might be slightly
    /// bigger than requested due to sector counts and alignment (e.g. 1MiB).
    /// If the requested size can't be allocated it results in an error.
    /// The maximal size can automatically be set by using 0 as size.
    fn resize(
        &self,
        size: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Set the `flags` property.
    ///
    /// See [`PartitionFlags`] for more information.
    fn set_flags(
        &self,
        flags: BitFlags<PartitionFlags>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets the partition name (label).
    fn set_name(
        &self,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets the partition type. See the "Type" property for a description of known partition types.
    fn set_type(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets the partition UUID (GPT only).
    #[zbus(name = "SetUUID")]
    fn set_uuid(
        &self,
        uuid: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Flags describing the partition.
    ///
    /// See [`PartitionFlags`] for more information.
    #[zbus(property)]
    fn flags(&self) -> error::Result<BitFlags<PartitionFlags>>;

    /// IsContained property
    #[zbus(property)]
    fn is_contained(&self) -> error::Result<bool>;

    ///  Whether the partition itself is a container for other partitions.
    ///
    /// For example, for dos partition tables,
    /// this applies to socalled extended partition (partitions of type 0x05, 0x0f or 0x85)
    /// containing socalled logical partitions.
    #[zbus(property)]
    fn is_container(&self) -> error::Result<bool>;

    /// Whether the partition is contained in another partition.
    ///
    /// See the [`Self::is_container`] for more information.
    #[zbus(property)]
    fn name(&self) -> error::Result<String>;

    /// Number of the partition in the partition table.
    #[zbus(property)]
    fn number(&self) -> error::Result<u32>;

    /// Offset of partition, in bytes.
    #[zbus(property)]
    fn offset(&self) -> error::Result<u64>;

    /// Size of partition, in bytes.
    #[zbus(property)]
    fn size(&self) -> error::Result<u64>;

    /// Object path of the [org.freedesktop.UDisks2.PartitionTable]
    /// object that the partition entry belongs to.
    #[zbus(property)]
    fn table(&self) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// Type of the partition.
    ///
    /// For `dos` partition tables, this string is a
    /// hexadecimal number e.g. `0x83` or `0xfd`.
    /// For `gpt` partition tables this is the UUID e.g.
    /// `ebd0a0a2-b9e5-4433-87c0-68b6b72699c7`.
    #[zbus(property)]
    fn type_(&self) -> error::Result<String>;

    /// UUID of the partition.
    ///
    /// Blank if not supported or unknown.
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> error::Result<String>;
}
