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

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.NVMe.Fabrics",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/NVMe"
)]
trait Fabrics {
    /// Disconnect method
    fn disconnect(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// HostID property
    #[zbus(property, name = "HostID")]
    fn host_id(&self) -> error::Result<Vec<u8>>;

    /// HostNQN property
    #[zbus(property, name = "HostNQN")]
    fn host_nqn(&self) -> error::Result<Vec<u8>>;

    /// Transport property
    #[zbus(property)]
    fn transport(&self) -> error::Result<String>;

    /// TransportAddress property
    #[zbus(property)]
    fn transport_address(&self) -> error::Result<Vec<u8>>;
}
