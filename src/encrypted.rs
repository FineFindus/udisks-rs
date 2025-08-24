//! This interface is used for [`org.freedesktop.UDisks2.Block`](crate::block) devices that contain encrypted data.
//! It provides methods for unlocking, locking, and managing encrypted block devices.

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.Encrypted",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Encrypted"
)]
pub trait Encrypted {
    /// Change the passphrase for the encrypted device.
    ///
    /// Changes the passphrase to `new_passphrase`. An existing passphrase is required.
    ///
    /// If `old_keyfile_contents` or `new_keyfile_contents` are given in `options`, they take
    /// precedence over the corresponding passphrase parameters individually.
    ///
    /// If the device is referenced in a system-wide configuration file (such as the
    /// `/etc/crypttab` file) and this configuration references the passphrase, it is not
    /// automatically updated.
    fn change_passphrase(
        &self,
        passphrase: &str,
        new_passphrase: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Locks the encrypted device.
    fn lock(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Resize the encrypted device.
    ///
    /// The device must be unlocked. The given size is the target size for the cleartext device.
    ///
    /// You need to specify either `passphrase` or `keyfile_contents` for LUKS 2 devices that
    /// don't have verified key loaded in kernel keyring.
    ///
    /// Available since version 2.8.0.
    fn resize(
        &self,
        size: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Tries to unlock the encrypted device using the provided passphrase.
    ///
    /// If the device is referenced in a system-wide configuration file (such as the
    /// `/etc/crypttab` file), then name, options and passphrase (if available) are used
    /// from that file after requesting additional authorization.
    ///
    /// If an empty passphrase should be used to unlock the device, it has to be
    /// passed using the `keyfile_contents` parameter. Empty string passed as
    /// `passphrase` means "Use the passphrase from the configuration file".
    ///
    /// If the device is removed without being locked (e.g. the user yanking the device
    /// or pulling the media out) the cleartext device will be cleaned up.
    ///
    /// If `read-only` is specified as an option, the device will be mounted in read-only mode.
    ///
    /// # Returns
    ///
    /// An object path to the unlocked object implementing the [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy) interface
    fn unlock(
        &self,
        passphrase: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// Configuration items belonging to the clear text device of this encrypted block and its children.
    ///
    /// This is also valid when this block device is currently locked and there is no clear text device for it.
    ///
    /// It works via the 'track-parents' options of [`BlockProxy::add_configuration_item`](crate::block::BlockProxy::add_configuration_item).
    #[zbus(property)]
    fn child_configuration(
        &self,
    ) -> error::Result<
        Vec<(
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// For an unlocked device, the object path of its cleartext device.
    #[zbus(property)]
    fn cleartext_device(&self) -> error::Result<zbus::zvariant::OwnedObjectPath>;

    /// If not blank, the type of the encryption used to encrypt this device.
    ///
    /// This is set during successful unlocking of an encrypted device. It is required for
    /// encryption types which can only be determined by decrypting the device (for example
    /// TCRYPT), but is used for all encryption types for consistency reasons.
    #[zbus(property)]
    fn hint_encryption_type(&self) -> error::Result<String>;

    /// Size of the metadata on the encrypted device in bytes.
    #[zbus(property)]
    fn metadata_size(&self) -> error::Result<u64>;
}
