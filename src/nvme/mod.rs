//! NVMe host management
//!
//! Extension of the top-level manager singleton object exposing
//! NVMe host management.

use zbus::proxy;

use crate::error;

pub mod controller;
pub mod fabrics;
pub mod namespace;

/// Transport options for [`NVMeProxy::connect`].
#[derive(Debug, serde::Serialize, zbus::zvariant::Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Transport {
    Rdma,
    Fc,
    Tcp,
    Loop,
}

#[proxy(
    interface = "org.freedesktop.UDisks2.Manager.NVMe",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Manager"
)]
pub trait NVMe {
    /// Create a transport connection to a remote system and create a NVMe over Fabrics controller
    /// for the NVMe subsystem specified by the `subsysnqn` option.
    ///
    /// By default, additional options are read from the system configuration file
    /// `/etc/nvme/config.json`. This follows the default behavior of `nvme-cli`.
    /// Use the `config` option to either specify a different config file or disable
    /// use of it altogether. The naming of the additional options are generally kept
    /// consistent with the JSON config file schema and any option specified acts as
    /// an override.
    ///
    /// Available options:
    ///
    /// - **`transport_svcid`** (`String`)
    ///   The transport service id. For transports using IP addressing (e.g. `rdma`)
    ///   this field is the port number. By default, the IP port number for the RDMA
    ///   transport is `4420`.
    ///
    /// - **`host_traddr`** (`String`)
    ///   The network address used on the host to connect to the Controller. For TCP,
    ///   this sets the source address on the socket.
    ///
    /// - **`host_iface`** (`String`)
    ///   The network interface used on the host to connect to the Controller (e.g.
    ///   `eth1`, `enp2s0`). This forces the connection to be made on a specific
    ///   interface instead of letting the system decide.
    ///
    /// - **`host_nqn`** (`Vec<u8>`)
    ///   Overrides the default Host NQN that identifies the NVMe Host.
    ///
    /// - **`host_id`** (`Vec<u8>`)
    ///   Overrides the default Host UUID.
    ///
    /// - **`config`** (`Option<String>`)
    ///   Use the specified JSON configuration file instead of the default file (see
    ///   above) or specify `"none"` to avoid reading any configuration file.
    ///
    /// - **`dhchap_key`** (`Vec<u8>`)
    ///   NVMe in-band authentication secret in ASCII format as described in the NVMe
    ///   2.0 specification. When not specified, the secret is by default read from
    ///   `/etc/nvme/hostkey`. In case that file does not exist, no in-band
    ///   authentication is attempted.
    ///
    /// - **`dhchap_ctrl_key`** (`Vec<u8>`)
    ///   NVMe in-band authentication controller secret for bi-directional
    ///   authentication. When not specified, no bi-directional authentication is
    ///   attempted.
    ///
    /// - **`nr_io_queues`** (`u32`)
    ///   The number of I/O queues.
    ///
    /// - **`nr_write_queues`** (`u32`)
    ///   Number of additional queues that will be used for write I/O.
    ///
    /// - **`nr_poll_queues`** (`u32`)
    ///   Number of additional queues that will be used for polling latency sensitive
    ///   I/O.
    ///
    /// - **`queue_size`** (`u32`)
    ///   Number of elements in the I/O queues.
    ///
    /// - **`keep_alive_tmo`** (`i32`)
    ///   The keep alive timeout (in seconds).
    ///
    /// - **`reconnect_delay`** (`i32`)
    ///   The delay (in seconds) before reconnect is attempted after a connect loss.
    ///
    /// - **`ctrl_loss_tmo`** (`i32`)
    ///   The controller loss timeout period (in seconds). A special value of `-1`
    ///   will cause reconnecting forever.
    ///
    /// - **`fast_io_fail_tmo`** (`i32`)
    ///   Fast I/O Fail timeout (in seconds).
    ///
    /// - **`tos`** (`String`)
    ///   Type of service.
    ///
    /// - **`duplicate_connect`** (`bool`)
    ///   Allow duplicated connections between same transport host and subsystem port.
    ///
    /// - **`disable_sqflow`** (`bool`)
    ///   Disables SQ flow control to omit head doorbell update for submission queues
    ///   when sending nvme completions.
    ///
    /// - **`hdr_digest`** (`bool`)
    ///   Generates/verifies header digest (TCP).
    ///
    /// - **`data_digest`** (`bool`)
    ///   Generates/verifies data digest (TCP).
    ///
    /// - **`tls`** (`bool`)
    ///   Enable TLS encryption (TCP).
    ///
    /// - **`hostsymname`** (`Vec<u8>`)
    ///   TP8010: NVMe host symbolic name.
    ///
    /// - **`keyring`** (`String`)
    ///   Keyring to store and lookup keys.
    ///
    /// - **`tls_key`** (`Vec<u8>`)
    ///   TLS PSK for the connection.
    ///
    /// Available since version 2.10.0.
    fn connect(
        &self,
        subsysnqn: &[u8],
        transport: Transport,
        transport_addr: &str,
        //TODO: use a struct for additional json overwrites?
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// Sets the system-wide Host ID string,
    ///
    /// Available since version 2.10.0.
    #[zbus(name = "SetHostID")]
    fn set_host_id(
        &self,
        hostid: &[u8],
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets the system-wide Host NQN string,
    ///
    /// Available since version 2.10.0.
    #[zbus(name = "SetHostNQN")]
    fn set_host_nqn(
        &self,
        hostnqn: &[u8],
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Host ID configured for the system.
    ///
    /// Reflects contents of the `/etc/nvme/hostid` file if present.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "HostID")]
    fn host_id(&self) -> error::Result<Vec<u8>>;

    /// Host NQN configured for the system.
    ///
    /// Reflects contents of the `/etc/nvme/hostnqn` file if present
    /// or uses autogenerated NQN value otherwise.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "HostNQN")]
    fn host_nqn(&self) -> error::Result<Vec<u8>>;
}
