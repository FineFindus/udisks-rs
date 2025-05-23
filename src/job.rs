//! # DBus interface proxy for: `org.freedesktop.UDisks2.Job`
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
    interface = "org.freedesktop.UDisks2.Job",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Jop"
)]
pub trait Job {
    /// Cancel method
    fn cancel(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Completed signal
    #[zbus(signal)]
    fn completed(&self, success: bool, message: &str) -> error::Result<()>;

    /// Bytes property
    #[zbus(property)]
    fn bytes(&self) -> error::Result<u64>;

    /// Cancelable property
    #[zbus(property)]
    fn cancelable(&self) -> error::Result<bool>;

    /// ExpectedEndTime property
    #[zbus(property)]
    fn expected_end_time(&self) -> error::Result<u64>;

    /// Objects property
    #[zbus(property)]
    fn objects(&self) -> error::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Operation property
    #[zbus(property)]
    fn operation(&self) -> error::Result<String>;

    /// Progress property
    #[zbus(property)]
    fn progress(&self) -> error::Result<f64>;

    /// ProgressValid property
    #[zbus(property)]
    fn progress_valid(&self) -> error::Result<bool>;

    /// Rate property
    #[zbus(property)]
    fn rate(&self) -> error::Result<u64>;

    /// StartTime property
    #[zbus(property)]
    fn start_time(&self) -> error::Result<u64>;

    /// StartedByUID property
    #[zbus(property, name = "StartedByUID")]
    fn started_by_uid(&self) -> error::Result<u32>;
}
