//! This interface is used for [`org.freedesktop.UDisks2.Block`](crate::block) devices that are loop devices

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.Loop",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Loop"
)]
pub trait Loop {
    /// Deletes the loop device.
    fn delete(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets the [`Self::autoclear`] property.
    ///
    /// Since this may require authentication and thus may
    /// fail, it is implemented this way instead of the property being
    /// writable.
    fn set_autoclear(
        &self,
        value: bool,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// When autoclear is enabled (`true`), the kernel will automatically
    /// clear the loop device when the last closer closes the device.
    /// This typically happens when the loop device is unmounted.
    #[zbus(property)]
    fn autoclear(&self) -> error::Result<bool>;

    /// Path to the file backing this loop device or blank if unknown.
    #[zbus(property)]
    fn backing_file(&self) -> error::Result<Vec<u8>>;

    /// ID of the user who set up the loop device, or 0 if it was set up
    /// by root or not through udisks
    #[zbus(property, name = "SetupByUID")]
    fn setup_by_uid(&self) -> error::Result<u32>;
}
