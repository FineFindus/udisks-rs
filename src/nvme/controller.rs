//! NVMe controller device
//!
//! This interface represents a controller device in a NVM subsystem.

use zbus::proxy;

use crate::error;

/// Indicates the type of sanitize action to take in [`ControllerProxy::sanitize_start`].
#[derive(Debug, serde::Serialize, zbus::zvariant::Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "kebab-case")]
pub enum SanitizeAction {
    BlockErase,
    /// Allows additional values via the `options` parameter to be set:
    ///  * `overwrite_pass_count` (`u8`) - Number of overwrite passes (1-15), defaults to 16 when not specified
    ///  * `overwrite_pattern` (type `u32`) - 32-bit pattern, defaults to zero if not specified
    ///  * `overwrite_invert_pattern` (type `bool`) - Indicates that the overwrite pattern should be inverted between passes
    Overwrite,
    CryptoErase,
}

/// Information about the most recent sanitize operation.
#[derive(Debug, serde::Serialize, zbus::zvariant::Type, zbus::zvariant::OwnedValue)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SanitizeStatus {
    /// The NVM subsystem has never been sanitized (or the status is unknown).
    NeverSanitized,
    /// Operation completed successfully.
    Success,
    /// The most recent sanitize operation failed.
    Failure,
    /// A sanitize operation is currently in progress.
    Inprogress,
}

#[derive(
    Debug, serde::Serialize, zbus::zvariant::Type, zbus::zvariant::OwnedValue, zbus::zvariant::Value,
)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SmartCriticalWarning {
    /// The available spare capacity has fallen below the threshold.
    Spare,
    /// A temperature is either greater than or equal to an over temperature threshold;
    /// or less than or equal to an under temperature threshold.
    Temperature,
    /// The NVM subsystem reliability has been degraded due to significant media
    /// related errors or any internal error that degrades NVM subsystem reliability.
    Degraded,
    /// All of the media has been placed in read only mode.
    ///
    /// Unrelated to the write protection state of a namespace.
    Readonly,
    /// The volatile memory backup device has failed.
    ///
    /// Only valid if the controller has a volatile memory backup solution
    VolatileMem,
    /// Persistent Memory Region has become read-only or unreliable.
    PmrReadonly,
}

/// SMART attributes obtained from [`ControllerProxy::smart_get_attributes`].
#[derive(Debug, zbus::zvariant::DeserializeDict, zbus::zvariant::Type)]
#[zvariant(signature = "dict")]
#[non_exhaustive]
pub struct SmartAttribute {
    /// Available Spare.
    ///
    /// A normalized percentage (0% to 100%) of the remaining spare capacity available.
    pub avail_spare: u8,
    /// Available Spare Threshold.
    ///
    /// A normalized percentage (0% to 100%) of the available spare threshold.
    pub spare_thresh: u8,
    /// Percentage Used.
    ///
    /// A vendor specific estimate of the percentage drive life used based
    /// on the actual usage and the manufacturer's prediction.
    /// A value of 100 indicates that the estimated endurance has been consumed,
    /// but may not indicate an NVM subsystem failure.
    /// The value is allowed to exceed 100.
    pub percent_used: u8,
    /// An estimated calculation of total data read in bytes based on calculation
    /// of data units read from the host.
    pub total_data_read: u64,
    /// An estimated calculation of total data written in bytes based on calculation
    /// of data units written by the host.
    pub total_data_written: u64,
    /// Amount of time the controller is busy with I/O commands, reported in minutes.
    pub ctrl_busy_time: u64,
    /// The number of power cycles.
    pub power_cycles: u64,
    /// The number of unsafe shutdowns as a result of a Shutdown Notification
    /// not received prior to loss of power.
    pub unsafe_shutdowns: u64,
    /// Media and Data Integrity Errors.
    ///
    /// The number of occurrences where the controller detected an unrecovered data
    /// integrity error (e.g. uncorrectable ECC, CRC checksum failure, or LBA tag mismatch).
    pub media_errors: u64,
    /// Number of Error Information Log Entries.
    ///
    /// The number of Error Information log entries over the life of the controller.
    pub num_err_log_entries: u64,
    /// Array of the current temperature reported by temperature sensors 1-8 in Kelvins
    /// or 0 when the particular sensor is not available.
    pub temp_sensors: Vec<u16>,
    /// Warning Composite Temperature Threshold (WCTEMP).
    ///
    /// Indicates the minimum Composite Temperature ([`ControllerProxy::smart_temperature`])
    /// value that indicates an overheating condition during which controller operation continues.
    pub wctemp: u16,
    /// Critical Composite Temperature Threshold (CCTEMP).
    ///
    /// indicates the minimum Composite Temperature ([`ControllerProxy::smart_temperature`])
    /// value that indicates a critical overheating condition (e.g., may prevent continued
    /// normal operation, possibility of data loss, automatic device shutdown,
    /// extreme performance throttling, or permanent damage).
    pub cctemp: u16,
    /// Warning Composite Temperature Time.
    ///
    /// The amount of time in minutes that the Composite Temperature
    /// ([`ControllerProxy::smart_temperature`]) is greater than or equal
    /// to the Warning Composite Temperature Threshold ([`Self::wctemp`])
    /// and less than the Critical Composite Temperature Threshold ([`Self::cctemp`]).
    pub warning_temp_time: u32,
    /// Critical Composite Temperature Time.
    ///
    /// The amount of time in minutes that the Composite Temperature
    /// ([`ControllerProxy::smart_temperature`]) is greater than or equal
    /// to the Critical Composite Temperature Threshold ([`Self::cctemp`]).
    pub critical_temp_time: u32,
}

#[derive(Debug, serde::Deserialize, zbus::zvariant::Type, zbus::zvariant::OwnedValue)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SmartSelftestStatus {
    /// Operation completed without error (or never ran).
    Success,
    /// Operation was aborted by a Device Self-test command.
    Aborted,
    /// Operation was aborted by a Controller Level Reset.
    CtrlReset,
    /// Operation was aborted due to a removal of a namespace
    /// from the namespace inventory.
    NsRemoved,
    /// Operation was aborted due to the processing of a
    /// Format NVM command.
    AbortedFormat,
    /// A fatal error or unknown test error occurred while the
    /// controller was executing the device self-test operation
    /// and the operation did not complete.
    FatalError,
    /// Operation completed with a segment that failed and the
    /// segment that failed is not known.
    UnknownSegFail,
    /// Operation completed with one or more failed segments.
    KnownSegFail,
    /// Operation was aborted for unknown reason.
    AbortedUnknown,
    /// Operation was aborted due to a sanitize operation.
    AbortedSanitize,
    /// Self-test operation is currently in progress.
    Inprogress,
}

/// Controller operating state.
///
/// Can be obtained from [`ControllerProxy::state`].
#[derive(Debug, serde::Deserialize, zbus::zvariant::Type, zbus::zvariant::OwnedValue)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum State {
    /// Controller is up and running.
    Live,
    New,
    Resetting,
    Connecting,
    Deleting,
    #[serde(rename(serialize = "deleting (no IO)"))]
    DeletingNoIO,
    Dead,
}

/// Type of selftest to run.
#[derive(Debug, serde::Serialize, zbus::zvariant::Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum SelfTestType {
    Short,
    Extended,
    VendorSpecific,
}

#[proxy(
    interface = "org.freedesktop.UDisks2.NVMe.Controller",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/NVMe"
)]
pub trait Controller {
    /// Starts a sanitize operation in the background.
    ///
    /// A sanitize operation alters all user data in the NVM subsystem such that recovery
    /// of any previous user data from any cache, the non-volatile media, or any Controller
    /// Memory Buffer is not possible. The scope of a sanitize operation is all locations
    /// in the NVM subsystem that are able to contain user data, including caches, Persistent
    /// Memory Regions, and unallocated or deallocated areas of the media.
    ///
    /// Once started, a sanitize operation is not able to be aborted and continues after
    /// a Controller Level Reset including across power cycles. Once the sanitize operation
    /// has run the media affected may not be immediately ready for use unless additional
    /// media modification mechanism is run. This is often vendor specific and also depends
    /// on the sanitize method (`action`) used.
    ///
    /// The sanitize operation is set to be executed with the No-Deallocate After Sanitize
    /// feature turned on, i.e. the controller shall not deallocate any user data as a result
    /// of successfully completing the sanitize operation.
    ///
    /// Additional option may be available when setting `action` to [`SanitizeAction::Overwrite`],
    /// refer to its documentation for more info.
    ///
    /// Available since version 2.10.0.
    fn sanitize_start(
        &self,
        action: SanitizeAction,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Get the SMART/Health Information attributes.
    ///
    /// Available since version 2.10.0.
    fn smart_get_attributes(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<SmartAttribute>;

    /// Aborts a running device selftest.
    ///
    /// Available since version 2.10.0.
    fn smart_selftest_abort(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Starts a device selftest operation on all active namespaces accessible through the controller
    /// at the time the operation is started.
    ///
    /// A device selftest operation is a diagnostic testing sequence that tests the integrity
    /// and functionality of the controller and may include testing of the media associated
    /// with namespaces.
    ///
    /// Note that the method returns immediately after the test has been started successfully
    /// and the operation is performed in the background.
    ///
    /// Available since version 2.10.0.
    fn smart_selftest_start(
        &self,
        type_: SelfTestType,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Reads SMART/Health Information from the NVMe controller and update relevant properties.
    ///
    /// Data in this interface are typically updated on every uevent or as a result of Asynchronous
    /// Event Notification.
    ///
    /// Available since version 2.10.0.
    fn smart_update(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// NVM subsystem unique controller identifier.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "ControllerID")]
    fn controller_id(&self) -> error::Result<u16>;

    /// FRU Globally Unique Identifier.
    ///
    /// Field-Replaceable Unit (FRU) is a physical component, device, or assembly that is able
    /// to be removed and replaced without having to replace the entire system.
    ///
    /// The FRU Globally Unique Identifier is a 128-bit value that is globally unique for a given
    /// Field Replaceable Unit (FRU). Value of "0" indicates this feature is not supported.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "FGUID")]
    fn fguid(&self) -> error::Result<String>;

    /// The major, minor, and micro version of the NVM Express base specification
    /// that the controller implementation supports.
    ///
    /// Note that some older devices (typically NVMe rev. lower than 1.2)
    /// may not always report this value.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "NVMeRevision")]
    fn nvme_revision(&self) -> error::Result<String>;

    /// Percent remaining or `-1` of unknown.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn sanitize_percent_remaining(&self) -> error::Result<i32>;

    /// The information about the most recent sanitize operation.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn sanitize_status(&self) -> error::Result<SanitizeStatus>;

    /// Critical warnings issued for the current state of the controller.
    ///
    /// An empty array indicates a healthy state. This is the primary health assessment
    /// property to watch for.
    ///
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn smart_critical_warning(&self) -> error::Result<Vec<SmartCriticalWarning>>;

    /// Amount of time the disk has been powered on (according to SMART data),
    /// or `0` if unknown.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn smart_power_on_hours(&self) -> error::Result<u64>;

    /// Percent remaining of the selftest operation,
    /// or `-1` if unknown.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn smart_selftest_percent_remaining(&self) -> error::Result<i32>;

    /// The status of the last self-test.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn smart_selftest_status(&self) -> error::Result<SmartSelftestStatus>;

    /// Temperature (in Kelvin) that represents the current composite temperature
    /// of the controller and associated namespaces or 0 if unknown.
    ///
    /// Values of the particular temperature sensors are exposed via the
    /// [`Self::smart_get_attributes`] method.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn smart_temperature(&self) -> error::Result<u16>;

    /// Point in time (seconds since the [`Unix Epoc`](std::time::UNIX_EPOCH) that the
    /// SMART/Health Information was updated, or `0` if never updated.
    ///
    /// The value of the other properties related to SMART are not meaningful if this property is 0.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn smart_updated(&self) -> error::Result<u64>;

    /// The controller operating state.
    ///
    /// Values other than [`State::Live`] may result in temporary refusal of
    /// any I/O and subsequent missing information provided by UDisks.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn state(&self) -> error::Result<State>;

    /// NVM Subsystem NVMe Qualified Name.
    ///
    /// Available since version 2.10.0.
    #[zbus(property, name = "SubsystemNQN")]
    fn subsystem_nqn(&self) -> error::Result<Vec<u8>>;

    /// Unallocated NVM capacity that is accessible by the controller.
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn unallocated_capacity(&self) -> error::Result<u64>;
}
