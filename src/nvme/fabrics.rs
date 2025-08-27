//! NVMe over Fabrics control interface
//!
//! Control interface for NVMe over Fabrics connected controllers.

use zbus::proxy;

use crate::error;

/// Transport values for [`FabricsProxy::transport`].
#[derive(Debug, serde::Deserialize, zbus::zvariant::Type, zbus::zvariant::OwnedValue)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Transport {
    /// PCI Express Transport
    Pcie,
    /// RDMA Transport
    Rdma,
    /// Fibre Channel Transport
    Fc,
    /// TCP Transport
    Tcp,
    /// Intra-host Transport (i.e loopback)
    Loop,
}

#[proxy(
    interface = "org.freedesktop.UDisks2.NVMe.Fabrics",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/NVMe"
)]
pub trait Fabrics {
    /// Disconnects and removes the NVMe over Fabrics controller.
    ///
    /// Available since version 2.10.0.
    fn disconnect(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Actual Host UUID used for the connection.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "HostID")]
    fn host_id(&self) -> error::Result<Vec<u8>>;

    /// Actual HostNQN used for the connection.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "HostNQN")]
    fn host_nqn(&self) -> error::Result<Vec<u8>>;

    /// Transport type the device is connected with.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn transport(&self) -> error::Result<Transport>;

    /// Network address of the controller.
    ///
    /// For transports using IP addressing (e.g. [`Transport::Rdma`])
    /// this should be an IP-based address (e.g. IPv4).
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn transport_address(&self) -> error::Result<Vec<u8>>;
}
