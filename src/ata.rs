//! This interface represents disk drives using the ATA command-set.
//!
//! Objects implementing this interface also implement the
//! [`org.freedesktop.UDisks2.Drive`](crate::drive) interface.

use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.Drive.Ata",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Drive"
)]
pub trait Ata {
    /// Get the current power mode status.
    ///
    /// This is implemented as a method call as it involves sending a command from the host to
    /// the drive and no change notification is available.
    ///
    /// The format of the returned state is the result obtained from sending the
    /// ATA command "CHECK POWER MODE" to the drive.
    fn pm_get_state(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<u8>;

    /// Force the drive to immediately enter the low power consumption __standby__ mode.
    ///
    /// This usually causes the drive to spin down. This is done by sending the ATA command
    /// "STANDBY IMMEDIATE" to the drive.
    fn pm_standby(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Force the drive to immediately wake up from standby mode.
    ///
    /// This exits the low power consumption __standby__ mode, usually causing the drive to spin up.
    /// This is done by reading data from the disk.
    fn pm_wakeup(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Perform a secure erase of the entire drive.
    ///
    /// Does all the necessary checks and preparations and then sends
    /// the "SECURITY ERASE UNIT" command to the drive. If the option `enhanced` is set to `true`,
    /// an enhanced secure erase is requested.
    ///
    /// **Warning: All data on the drive will be irrevocably erased.**
    ///
    /// This operation takes either [`Self::security_erase_unit_minutes`] or
    /// [`Self::security_enhanced_erase_unit_minutes`] minutes to complete depending on whether
    /// the `enhanced` option is `true`.
    ///
    /// # Arguments
    /// * `options` - Options, including:
    ///   - `enhanced`: Set to `true` for enhanced secure erase
    fn security_erase_unit(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Get the SMART attributes from the drive.
    ///
    /// # Arguments
    /// * `options` - Options, including:
    ///   - `nowakeup` (type 'b'): Don't wake up sleeping drives
    #[allow(clippy::type_complexity)]
    fn smart_get_attributes(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<
        Vec<(
            u8,
            String,
            u16,
            i32,
            i32,
            i32,
            i64,
            i32,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// Abort a running SMART selftest.
    fn smart_selftest_abort(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Start a SMART selftest.
    ///
    /// The method returns immediately after the test has been started successfully.
    fn smart_selftest_start(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Enable or disable SMART for the drive.
    ///
    /// This setting is stored in the non-volatile memory in the drive itself and does
    /// not need to be refreshed every time the drive is powered on or connected.
    ///
    /// Since this may require authentication and thus may fail, it is
    /// implemented as a method instead of the [`Self::smart_enabled`] property being writable.
    fn smart_set_enabled(
        &self,
        value: bool,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Read SMART data from the drive and update relevant properties.
    ///
    /// # Errors
    ///
    /// If the option `nowakeup` is given and the disk is in a sleeping
    /// state, the error [`error::Error::WouldWakeup`] is returned.
    ///
    /// # Arguments
    /// * `options` - Options, including:
    ///   - `nowakeup` (type 'b'): Don't wake up sleeping drives
    fn smart_update(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Whether Automatic Acoustic Management (AAM) is enabled.
    #[zbus(property)]
    fn aam_enabled(&self) -> error::Result<bool>;

    /// Whether the drive supports Automatic Acoustic Management (AAM).
    #[zbus(property)]
    fn aam_supported(&self) -> error::Result<bool>;

    /// The vendor-recommended AAM value (or 0 if AAM is not supported).
    #[zbus(property)]
    fn aam_vendor_recommended_value(&self) -> error::Result<i32>;

    /// Whether Advanced Power Management (APM) is enabled.
    #[zbus(property)]
    fn apm_enabled(&self) -> error::Result<bool>;

    /// Whether the drive supports Advanced Power Management (APM).
    #[zbus(property)]
    fn apm_supported(&self) -> error::Result<bool>;

    /// Whether power management is enabled.
    #[zbus(property)]
    fn pm_enabled(&self) -> error::Result<bool>;

    /// Whether the drive supports power management.
    #[zbus(property)]
    fn pm_supported(&self) -> error::Result<bool>;

    /// Whether the read look-ahead is enabled (or `false` if not supported).
    ///
    /// Available since version 2.1.7.
    #[zbus(property)]
    fn read_lookahead_enabled(&self) -> error::Result<bool>;

    /// Whether the drive supports configuring the read look-ahead.
    ///
    /// Available since version 2.1.7.
    #[zbus(property)]
    fn read_lookahead_supported(&self) -> error::Result<bool>;

    /// The estimated amount of minutes it takes to complete the "SECURITY ERASE UNIT"
    /// command with enhanced mode specified or 0 if enhanced erase is not available.
    ///
    /// If set to 510 it means that it takes at least 508 minutes to complete the operation.
    #[zbus(property)]
    fn security_enhanced_erase_unit_minutes(&self) -> error::Result<i32>;

    /// The estimated amount of minutes it takes to complete the "SECURITY ERASE UNIT"
    /// command or 0 if this command is not available.
    ///
    /// If set to 510 it means that it takes at least 508 minutes to complete the operation.
    #[zbus(property)]
    fn security_erase_unit_minutes(&self) -> error::Result<i32>;

    /// Whether the security unit is frozen.
    #[zbus(property)]
    fn security_frozen(&self) -> error::Result<bool>;

    /// Whether SMART is enabled.
    #[zbus(property)]
    fn smart_enabled(&self) -> error::Result<bool>;

    /// Whether the disk is about to fail according to SMART data.
    ///
    /// This value is read from the disk itself and does not include any interpretation.
    #[zbus(property)]
    fn smart_failing(&self) -> error::Result<bool>;

    /// The number of attributes that have failed in the past or -1 if unknown.
    #[zbus(property)]
    fn smart_num_attributes_failed_in_the_past(&self) -> error::Result<i32>;

    /// The number of attributes failing right now or -1 if unknown.
    #[zbus(property)]
    fn smart_num_attributes_failing(&self) -> error::Result<i32>;

    /// The number of bad sectors (i.e. pending and reallocated) or -1 if unknown.
    #[zbus(property)]
    fn smart_num_bad_sectors(&self) -> error::Result<i64>;

    /// The amount of time the disk has been powered on (according to SMART data) or 0 if unknown.
    #[zbus(property)]
    fn smart_power_on_seconds(&self) -> error::Result<u64>;

    /// The percent remaining of the current self-test or -1 if unknown.
    #[zbus(property)]
    fn smart_selftest_percent_remaining(&self) -> error::Result<i32>;

    /// The status of the last self-test.
    #[zbus(property)]
    fn smart_selftest_status(&self) -> error::Result<String>;

    /// Whether the drive supports SMART.
    #[zbus(property)]
    fn smart_supported(&self) -> error::Result<bool>;

    /// The temperature (in Kelvin) of the disk according to SMART data or 0 if unknown.
    #[zbus(property)]
    fn smart_temperature(&self) -> error::Result<f64>;

    /// The point in time (seconds since the [`Unix Epoc`](std::time::UNIX_EPOCH) that the SMART status was updated
    /// or 0 if never updated.
    ///
    /// The value of the other properties related to SMART are not meaningful if this
    /// property is 0.
    #[zbus(property)]
    fn smart_updated(&self) -> error::Result<u64>;

    /// Whether the write-cache is enabled (or `false` if not supported).
    ///
    /// Available since version 2.0.0.
    #[zbus(property)]
    fn write_cache_enabled(&self) -> error::Result<bool>;

    /// Whether the drive supports configuring the write cache.
    ///
    /// Available since version 2.0.0.
    #[zbus(property)]
    fn write_cache_supported(&self) -> error::Result<bool>;
}
