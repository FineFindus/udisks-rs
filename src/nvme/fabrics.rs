//! # DBus interface proxy for: `org.freedesktop.UDisks2.NVMe.Fabrics`
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
    interface = "org.freedesktop.UDisks2.NVMe.Fabrics",
    assume_defaults = true
)]
trait Fabrics {
    /// Disconnect method
    fn disconnect(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// HostID property
    #[dbus_proxy(property, name = "HostID")]
    fn host_id(&self) -> zbus::Result<Vec<u8>>;

    /// HostNQN property
    #[dbus_proxy(property, name = "HostNQN")]
    fn host_nqn(&self) -> zbus::Result<Vec<u8>>;

    /// Transport property
    #[dbus_proxy(property)]
    fn transport(&self) -> zbus::Result<String>;

    /// TransportAddress property
    #[dbus_proxy(property)]
    fn transport_address(&self) -> zbus::Result<Vec<u8>>;
}
