use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use zbus::{proxy, zvariant::Type};

use crate::error;

///Flags describing the partition.
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
    /// Delete method
    fn delete(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Resize method
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

    /// SetName method
    fn set_name(
        &self,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// SetType method
    fn set_type(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// SetUUID method
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

    /// IsContainer property
    #[zbus(property)]
    fn is_container(&self) -> error::Result<bool>;

    /// Name property
    #[zbus(property)]
    fn name(&self) -> error::Result<String>;

    /// Number property
    #[zbus(property)]
    fn number(&self) -> error::Result<u32>;

    /// Offset property
    #[zbus(property)]
    fn offset(&self) -> error::Result<u64>;

    /// Size property
    #[zbus(property)]
    fn size(&self) -> error::Result<u64>;

    /// Table property
    #[zbus(property)]
    fn table(&self) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// Type property
    #[zbus(property)]
    fn type_(&self) -> error::Result<String>;

    /// UUID property
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> error::Result<String>;
}
