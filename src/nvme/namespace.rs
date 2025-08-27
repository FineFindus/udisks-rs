//! NVMe namespace device
//!
//! This interface represents a namespace device in a NVMe subsystem.

use zbus::proxy;

use crate::error;

#[derive(
    Debug,
    zbus::zvariant::Type,
    serde::Deserialize,
    zbus::zvariant::OwnedValue,
    zbus::zvariant::Value,
)]
pub struct LBAFormat {
    /// LBA Data Size in bytes.
    pub size: u16,
    /// Number of metadata bytes provided per LBA.
    pub metadata_size: u16,
    /// Relative performance relative to other formats returned from
    /// [`NamespaceProxy::lbaformats`].
    pub performance: LBAPerformance,
}

#[derive(
    Debug,
    zbus::zvariant::Type,
    serde_repr::Deserialize_repr,
    zbus::zvariant::OwnedValue,
    zbus::zvariant::Value,
)]
#[repr(u8)]
#[non_exhaustive]
pub enum LBAPerformance {
    /// Unknown relative performance index.
    Unkown,
    /// Best performance.
    Best,
    /// Better performance.
    Better,
    /// Good performance.
    Good,
    /// Degraded performance.
    Degraded,
}

#[proxy(
    interface = "org.freedesktop.UDisks2.NVMe.Namespace",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/NVMe"
)]
pub trait Namespace {
    /// Performs low level format of the NVM media,
    /// destroying all data and metadata in the current namespace.
    ///
    /// The optional `lba_data_size` parameter indicates the LBA Data Size
    /// in bytes to use (see the related [`Self::lbaformats`]) and similarly the
    /// `metadata_size` parameter denotes the number of metadata bytes provided per LBA.
    /// If not specified, the current active format is used.
    ///
    /// The optional `secure_erase` parameter can be used to perform secure erase - valid values are
    /// `user_data` where the user data are overwritten by a pattern, and `crypto_erase`
    /// which removes the encryption key with which the user data was previously encrypted.
    ///
    /// This call blocks until the format operation has finished.
    ///
    /// Available since version 2.10.0.
    fn format_namespace(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// IEEE Extended Unique Identifier.
    ///
    /// A 64-bit value that is globally unique and assigned to the namespace
    /// when the namespace is created.
    /// Remains fixed throughout the life of the namespace.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "EUI64")]
    fn eui64(&self) -> error::Result<String>;

    /// Percent remaining of a running format operation or -1 if unknown
    /// (e.g. not reported by the drive).
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn format_percent_remaining(&self) -> error::Result<i32>;

    /// Actual LBA data size, the metadata size and the relative performance index
    /// that the namespace has been formatted with.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "FormattedLBASize")]
    fn formatted_lbasize(&self) -> error::Result<LBAFormat>;

    /// List of LBA formats supported by the controller.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "LBAFormats")]
    fn lbaformats(&self) -> error::Result<Vec<LBAFormat>>;

    /// Namespace Globally Unique Identifier.
    ///
    /// A 128-bit value that is globally unique and assigned to
    /// the namespace when the namespace is created.
    /// Remains fixed throughout the life of the namespace.
    ///
    /// Available since version 2.10.0
    #[zbus(property, name = "NGUID")]
    fn nguid(&self) -> error::Result<String>;

    /// Namespace Identifier
    ///
    /// Available since version 2.10.0
    #[zbus(property, name = "NSID")]
    fn nsid(&self) -> error::Result<u32>;

    /// Maximum number of logical blocks that may be allocated in the namespace.
    ///
    /// The number of logical blocks is based on the formatted
    /// LBA size (see [`Self::formatted_lbasize`]).
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn namespace_capacity(&self) -> error::Result<u64>;

    /// Total size of the namespace in logical blocks.
    ///
    /// The number of logical blocks is based on the formatted
    /// LBA size (see [`Self::formatted_lbasize`]).
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn namespace_size(&self) -> error::Result<u64>;

    /// Current number of logical blocks allocated in the namespace.
    ///
    /// This value is less than or equal to the [`Self::namespace_capacity`].
    /// The number of logical blocks is based on the formatted LBA
    /// size (see [`Self::formatted_lbasize`]).
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn namespace_utilization(&self) -> error::Result<u64>;

    /// Namespace UUID.
    ///
    /// Contains a 128-bit Universally Unique Identifier (UUID)
    /// as specified in RFC 4122.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> error::Result<String>;

    /// [World Wide Name](http://en.wikipedia.org/wiki/World_Wide_Name) of the namespace
    /// or blank if unknown.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "WWN")]
    fn wwn(&self) -> error::Result<String>;
}
