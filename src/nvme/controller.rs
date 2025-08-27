//! NVMe controller device
//!
//! This interface represents a controller device in a NVM subsystem.

use zbus::proxy;

use crate::error;

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
        action: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Get the SMART/Health Information attributes.
    ///
    /// Available since version 2.10.0.
    fn smart_get_attributes(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

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
        type_: &str,
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
    fn sanitize_status(&self) -> error::Result<String>;

    /// Critical warnings issued for the current state of the controller.
    ///
    /// An empty array indicates a healthy state. This is the primary health assessment
    /// property to watch for.
    ///
    ///
    /// Available since version 2.10.0.
    #[zbus(property)]
    fn smart_critical_warning(&self) -> error::Result<Vec<String>>;

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
    fn smart_selftest_status(&self) -> error::Result<String>;

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
    fn state(&self) -> error::Result<String>;

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
