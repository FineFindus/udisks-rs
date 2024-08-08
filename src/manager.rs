//! # DBus interface proxy for: `org.freedesktop.UDisks2.Manager`
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
    interface = "org.freedesktop.UDisks2.Manager",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Manager"
)]
trait Manager {
    /// CanCheck method
    fn can_check(&self, type_: &str) -> error::Result<(bool, String)>;

    /// CanFormat method
    fn can_format(&self, type_: &str) -> error::Result<(bool, String)>;

    /// CanRepair method
    fn can_repair(&self, type_: &str) -> error::Result<(bool, String)>;

    /// CanResize method
    fn can_resize(&self, type_: &str) -> error::Result<(bool, u64, String)>;

    /// EnableModule method
    fn enable_module(&self, name: &str, enable: bool) -> error::Result<()>;

    /// EnableModules method
    #[deprecated(note = "Use EnableModule instead")]
    fn enable_modules(&self, enable: bool) -> error::Result<()>;

    /// GetBlockDevices method
    fn get_block_devices(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// LoopSetup method
    fn loop_setup(
        &self,
        fd: zbus::zvariant::Fd<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// MDRaidCreate method
    #[zbus(name = "MDRaidCreate")]
    fn mdraid_create(
        &self,
        blocks: &[zbus::zvariant::ObjectPath<'_>],
        level: &str,
        name: &str,
        chunk: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// ResolveDevice method
    fn resolve_device(
        &self,
        devspec: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// DefaultEncryptionType property
    #[zbus(property)]
    fn default_encryption_type(&self) -> error::Result<String>;

    /// SupportedEncryptionTypes property
    #[zbus(property)]
    fn supported_encryption_types(&self) -> error::Result<Vec<String>>;

    /// SupportedFilesystems property
    #[zbus(property)]
    fn supported_filesystems(&self) -> error::Result<Vec<String>>;

    /// Version property
    #[zbus(property)]
    fn version(&self) -> error::Result<String>;
}
