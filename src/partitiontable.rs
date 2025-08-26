//! Block device containing a partition table

//!
//! This interface is used for [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy)
//! devices that contain a partition table.

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.PartitionTable",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/PartitionTable"
)]
pub trait PartitionTable {
    /// Creates a new partition.
    ///
    /// Note that the created partition won't necessarily be created
    /// at the exact `offset` but slightly behind due to disk geometry
    /// and other alignment constraints (e.g. 1MiB alignment).
    ///
    /// The newly created partition may also end up being slightly
    /// larger than the requested `size` bytes for the same reasons.
    /// The maximal size can be automatically set by using `0` as size.
    ///
    /// For `dos` partition tables, the partition type can be
    /// set with the `partition-type` option. Possible values are: "primary",
    /// "extended" or "logical".
    ///
    /// An optional parameter `partition-uuid` denotes
    /// the partition UUID to set for the newly created partition (GPT only).
    ///
    /// The newly created partition will be wiped of known filesystem
    /// signatures using the `wipefs` command.
    fn create_partition(
        &self,
        offset: u64,
        size: u64,
        type_: &str,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// This is a combination of
    /// [`Self::create_partition`] and [`BlockProxy::format`](crate::block::BlockProxy::format).
    ///
    /// After creating the partition, the resulting block device is formatted.
    #[allow(clippy::too_many_arguments)]
    fn create_partition_and_format(
        &self,
        offset: u64,
        size: u64,
        type_: &str,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        format_type: &str,
        format_options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// List of object paths of the
    /// [`org.freedesktop.UDisks2.Partition`](crate::partition::PartitionProxy)
    /// objects that belongs to this partition table.
    ///
    /// Available since version 2.7.2
    #[zbus(property)]
    fn partitions(&self) -> error::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Type of partition table detected.
    #[zbus(property)]
    fn type_(&self) -> error::Result<String>;
}
