//! # DBus interface proxy for: `org.freedesktop.UDisks2.Encrypted`
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
    interface = "org.freedesktop.UDisks2.Encrypted",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Encrypted"
)]
pub trait Encrypted {
    /// ChangePassphrase method
    fn change_passphrase(
        &self,
        passphrase: &str,
        new_passphrase: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Lock method
    fn lock(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Resize method
    fn resize(
        &self,
        size: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Unlock method
    fn unlock(
        &self,
        passphrase: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// ChildConfiguration property
    #[zbus(property)]
    fn child_configuration(
        &self,
    ) -> error::Result<
        Vec<(
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// CleartextDevice property
    #[zbus(property)]
    fn cleartext_device(&self) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// HintEncryptionType property
    #[zbus(property)]
    fn hint_encryption_type(&self) -> error::Result<String>;

    /// MetadataSize property
    #[zbus(property)]
    fn metadata_size(&self) -> error::Result<u64>;
}
