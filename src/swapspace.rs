//! Block device containing swap data
//!
//! This interface is used for [`org.freedesktop.UDisks2.Block`](crate::block::BlockProxy)
//! devices that contain swap space.

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.Swapspace",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Swapspace"
)]
pub trait Swapspace {
    /// Set label for the swap device.
    fn set_label(
        &self,
        label: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Set UUID for the swap device.
    #[zbus(name = "SetUUID")]
    fn set_uuid(
        &self,
        uuid: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Activates the swap device.
    fn start(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Deactivates the swap device.
    fn stop(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Whether the device is currently in use by the OS.
    #[zbus(property)]
    fn active(&self) -> error::Result<bool>;
}
