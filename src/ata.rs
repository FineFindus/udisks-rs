//! # DBus interface proxy for: `org.freedesktop.UDisks2.Drive.Ata`
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
    interface = "org.freedesktop.UDisks2.Drive.Ata",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Drive"
)]
pub trait Ata {
    /// PmGetState method
    fn pm_get_state(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<u8>;

    /// PmStandby method
    fn pm_standby(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// PmWakeup method
    fn pm_wakeup(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// SecurityEraseUnit method
    fn security_erase_unit(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// SmartGetAttributes method
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

    /// SmartSelftestAbort method
    fn smart_selftest_abort(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// SmartSelftestStart method
    fn smart_selftest_start(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// SmartSetEnabled method
    fn smart_set_enabled(
        &self,
        value: bool,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// SmartUpdate method
    fn smart_update(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// AamEnabled property
    #[zbus(property)]
    fn aam_enabled(&self) -> error::Result<bool>;

    /// AamSupported property
    #[zbus(property)]
    fn aam_supported(&self) -> error::Result<bool>;

    /// AamVendorRecommendedValue property
    #[zbus(property)]
    fn aam_vendor_recommended_value(&self) -> error::Result<i32>;

    /// ApmEnabled property
    #[zbus(property)]
    fn apm_enabled(&self) -> error::Result<bool>;

    /// ApmSupported property
    #[zbus(property)]
    fn apm_supported(&self) -> error::Result<bool>;

    /// PmEnabled property
    #[zbus(property)]
    fn pm_enabled(&self) -> error::Result<bool>;

    /// PmSupported property
    #[zbus(property)]
    fn pm_supported(&self) -> error::Result<bool>;

    /// ReadLookaheadEnabled property
    #[zbus(property)]
    fn read_lookahead_enabled(&self) -> error::Result<bool>;

    /// ReadLookaheadSupported property
    #[zbus(property)]
    fn read_lookahead_supported(&self) -> error::Result<bool>;

    /// SecurityEnhancedEraseUnitMinutes property
    #[zbus(property)]
    fn security_enhanced_erase_unit_minutes(&self) -> error::Result<i32>;

    /// SecurityEraseUnitMinutes property
    #[zbus(property)]
    fn security_erase_unit_minutes(&self) -> error::Result<i32>;

    /// SecurityFrozen property
    #[zbus(property)]
    fn security_frozen(&self) -> error::Result<bool>;

    /// SmartEnabled property
    #[zbus(property)]
    fn smart_enabled(&self) -> error::Result<bool>;

    /// SmartFailing property
    #[zbus(property)]
    fn smart_failing(&self) -> error::Result<bool>;

    /// SmartNumAttributesFailedInThePast property
    #[zbus(property)]
    fn smart_num_attributes_failed_in_the_past(&self) -> error::Result<i32>;

    /// SmartNumAttributesFailing property
    #[zbus(property)]
    fn smart_num_attributes_failing(&self) -> error::Result<i32>;

    /// SmartNumBadSectors property
    #[zbus(property)]
    fn smart_num_bad_sectors(&self) -> error::Result<i64>;

    /// SmartPowerOnSeconds property
    #[zbus(property)]
    fn smart_power_on_seconds(&self) -> error::Result<u64>;

    /// SmartSelftestPercentRemaining property
    #[zbus(property)]
    fn smart_selftest_percent_remaining(&self) -> error::Result<i32>;

    /// SmartSelftestStatus property
    #[zbus(property)]
    fn smart_selftest_status(&self) -> error::Result<String>;

    /// SmartSupported property
    #[zbus(property)]
    fn smart_supported(&self) -> error::Result<bool>;

    /// SmartTemperature property
    #[zbus(property)]
    fn smart_temperature(&self) -> error::Result<f64>;

    /// SmartUpdated property
    #[zbus(property)]
    fn smart_updated(&self) -> error::Result<u64>;

    /// WriteCacheEnabled property
    #[zbus(property)]
    fn write_cache_enabled(&self) -> error::Result<bool>;

    /// WriteCacheSupported property
    #[zbus(property)]
    fn write_cache_supported(&self) -> error::Result<bool>;
}
