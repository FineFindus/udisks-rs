//! # DBus interface proxy for: `org.freedesktop.UDisks2.NVMe.Namespace`
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

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.UDisks2.NVMe.Namespace",
    assume_defaults = true
)]
trait Namespace {
    /// FormatNamespace method
    fn format_namespace(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// EUI64 property
    #[dbus_proxy(property, name = "EUI64")]
    fn eui64(&self) -> zbus::Result<String>;

    /// FormatPercentRemaining property
    #[dbus_proxy(property)]
    fn format_percent_remaining(&self) -> zbus::Result<i32>;

    /// FormattedLBASize property
    #[dbus_proxy(property, name = "FormattedLBASize")]
    fn formatted_lbasize(&self) -> zbus::Result<(u16, u16, u8)>;

    /// LBAFormats property
    #[dbus_proxy(property, name = "LBAFormats")]
    fn lbaformats(&self) -> zbus::Result<Vec<(u16, u16, u8)>>;

    /// NGUID property
    #[dbus_proxy(property, name = "NGUID")]
    fn nguid(&self) -> zbus::Result<String>;

    /// NSID property
    #[dbus_proxy(property, name = "NSID")]
    fn nsid(&self) -> zbus::Result<u32>;

    /// NamespaceCapacity property
    #[dbus_proxy(property)]
    fn namespace_capacity(&self) -> zbus::Result<u64>;

    /// NamespaceSize property
    #[dbus_proxy(property)]
    fn namespace_size(&self) -> zbus::Result<u64>;

    /// NamespaceUtilization property
    #[dbus_proxy(property)]
    fn namespace_utilization(&self) -> zbus::Result<u64>;

    /// UUID property
    #[dbus_proxy(property, name = "UUID")]
    fn uuid(&self) -> zbus::Result<String>;

    /// WWN property
    #[dbus_proxy(property, name = "WWN")]
    fn wwn(&self) -> zbus::Result<String>;
}
