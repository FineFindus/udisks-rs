//! # DBus interface proxy for: `org.freedesktop.UDisks2.PartitionTable`
//!
//! This code was generated by `zbus-xmlgen` `4.0.0` from DBus introspection data.
//! Source: `org.freedesktop.UDisks2.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus2.github.io/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.PartitionTable",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/PartitionTable"
)]
trait PartitionTable {
    /// CreatePartition method
    fn create_partition(
        &self,
        offset: u64,
        size: u64,
        type_: &str,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// CreatePartitionAndFormat method
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

    /// Partitions property
    #[zbus(property)]
    fn partitions(&self) -> error::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Type property
    #[zbus(property)]
    fn type_(&self) -> error::Result<String>;
}
