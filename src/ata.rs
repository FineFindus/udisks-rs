//! This interface represents disk drives using the ATA command-set.
//!
//! Objects implementing this interface also implement the
//! [`org.freedesktop.UDisks2.Drive`](crate::drive) interface.

use core::str;
use std::str::FromStr;

use serde::{Deserialize, de::IntoDeserializer};
use zbus::{
    proxy,
    zvariant::{OwnedValue, Value},
};

use crate::error;

/// Power mode status of a drive.
///
/// This is typically reported as "Drive is spun down" if the mode is [`PowerModeStatus::Standby`]
/// and "Drive is spun up" otherwise.
#[derive(Debug, zbus::zvariant::Type, serde_repr::Deserialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum PowerModeStatus {
    /// Standby mode.
    ///
    /// This is typically reported as "Drive is spun down".
    Standby = 0x00,
    Idle = 0x80,
    /// Active/Idle
    Active = 0xFF,
}

#[derive(Debug, zbus::zvariant::Type, serde::Deserialize)]
pub struct SmartAttribute {
    /// Attribute Identifier.
    pub id: u8,
    /// The identifier as a string.
    ///
    /// Should be used as the authoritative identifier for the attribute
    /// since it is derived from the numerical [`Self::id`] and the disk's `IDENTIFY` data and thus
    /// handles ID collisions between drives of different make and model.
    pub name: String,
    /// 16-bit attribute flags (bit `0` is prefail/oldage, bit `1 is online/offline).
    pub flags: u16,
    /// The current value or `-1` if unknown.
    pub value: i32,
    /// The worst value or `-1` if unknown.
    pub worst: i32,
    /// The threshold or `-1` if unknown.
    pub threshold: i32,
    /// An interpretation of the value - must be ignored if [`Self::pretty_unit`] is `0`.
    pub pretty: i64,
    /// The unit of the [`Self::pretty`] value.
    pub pretty_unit: PrettyUnit,
    /// Currently unused, intended for future expansion.
    pub expansion: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
}

/// The unit of the [`SmartAttribute::pretty`] value.
#[derive(Debug, zbus::zvariant::Type, serde::Deserialize)]
#[repr(i32)]
#[non_exhaustive]
pub enum PrettyUnit {
    Unknown = 0,
    Dimentionless = 1,
    Milliseconds = 2,
    Sectors = 3,
    Millikelvin = 4,
}

/// Type of test to run.
#[derive(Debug, serde::Serialize, zbus::zvariant::Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SelfTestType {
    /// Short self-test
    Short,
    /// Extended self-test
    Extended,
    /// Conveyance self-test
    Conveyance,
    /// Offline self-test (since 2.11.0)
    Offline,
}

/// Indicates the result of a SMART selftest.
#[derive(Debug, serde::Deserialize, zbus::zvariant::Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SelfTestStatus {
    /// Last self-test was a success (or never ran)
    Success,
    /// Last self-test was aborted
    Aborted,
    /// Last self-test was interrupted
    Interrupted,
    /// Last self-test did not complete
    Fatal,
    /// Last self-test failed (Unknown)
    ErrorUnknown,
    /// Last self-test failed (Electrical)
    ErrorElectrical,
    /// Last self-test failed (Servo)
    ErrorServo,
    /// Last self-test failed (Read)
    ErrorRead,
    /// Last self-test failed (Damage)
    ErrorHandling,
    /// Self-test is currently in progress
    Inprogress,
}

impl FromStr for SelfTestStatus {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl TryFrom<Value<'_>> for SelfTestStatus {
    type Error = <String as TryFrom<Value<'static>>>::Error;

    fn try_from(value: Value<'_>) -> Result<Self, Self::Error> {
        let val: String = value.downcast_ref()?;
        Self::from_str(&val).map_err(|_| zbus::zvariant::Error::IncorrectType)
    }
}

impl TryFrom<OwnedValue> for SelfTestStatus {
    type Error = <String as TryFrom<OwnedValue>>::Error;

    fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
        Self::try_from(Into::<Value<'_>>::into(v))
    }
}

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
    ) -> error::Result<PowerModeStatus>;

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
    fn smart_get_attributes(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<Vec<SmartAttribute>>;

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
        type_: SelfTestType,
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
    fn smart_selftest_status(&self) -> error::Result<SelfTestStatus>;

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
