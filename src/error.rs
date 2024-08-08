use std::{convert::Infallible, fmt::Display};

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for `UDisks2`.
///
/// Possible errors and their corresponding D-Bus error names.
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// The operation failed.
    Failed,
    /// The operation was cancelled.
    Cancelled,
    /// The operation has already been cancelled.
    AlreadyCancelled,
    /// Not authorized to perform the requested operation.
    NotAuthorized,
    /// Like [`Error::NotAuthorized`] but authorization can be obtained through e.g. authentication.
    NotAuthorizedCanObtain,
    /// Like [`Error::NotAuthorized`] but an authentication was shown and the user dismissed it.
    NotAuthorizedDismissed,
    /// The device is already mounted.
    AlreadyMounted,
    /// The device is not mounted.
    NotMounted,
    /// Not permitted to use the requested option.
    OptionNotPermitted,
    /// The device is mounted by another user.
    MountedByOtherUser,
    /// The device is already unmounting.
    AlreadyUnmounting,
    /// The operation is not supported due to missing driver/tool support.
    NotSupported,
    /// The operation timed out.
    TimedOut,
    /// The operation would wake up a disk that is in a deep-sleep state.
    WouldWakeup,
    /// Attempting to unmount a device that is busy.
    DeviceBusy,
    Iscsi(Iscsi),
    /// The operation failed due to an [`zbus::Error`].
    Zbus(zbus::Error),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Iscsi {
    DaemonTransportFailed,
    HostNotFound,
    Idmb,
    LoginFailed,
    LoginAuthFailed,
    LoginFatal,
    LogoutFailed,
    NoFirmware,
    NoObjectsFound,
    NotConnected,
    TransportFailed,
    UnknownDiscoveryType,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Failed => write!(f, "The operation failed"),
            Error::Cancelled => write!(f, "The operation was cancelled."),
            Error::AlreadyCancelled => write!(f, "The operation has already been cancelled."),
            Error::NotAuthorized => write!(f, "Not authorized to perform the requested operation."),
            Error::NotAuthorizedCanObtain => write!(f, "Like `Error::NotAuthorized` but authorization can be obtained through e.g. authentication."),
            Error::NotAuthorizedDismissed => write!(f, "Like `Error::NotAuthorized` but an authentication was shown and the user dismissed it."),
            Error::AlreadyMounted => write!(f, "The device is already mounted."),
            Error::NotMounted => write!(f, "The device is not mounted."),
            Error::OptionNotPermitted => write!(f, "Not permitted to use the requested option."),
            Error::MountedByOtherUser => write!(f, "The device is mounted by another user."),
            Error::AlreadyUnmounting => write!(f, "The device is already unmounting."),
            Error::NotSupported => write!(f, "The operation is not supported due to missing driver/tool support."),
            Error::TimedOut => write!(f, "The operation timed out."),
            Error::WouldWakeup => write!(f, "The operation would wake up a disk that is in a deep-sleep state."),
            Error::DeviceBusy => write!(f, "Attempting to unmount a device that is busy."),
            Error::Iscsi(_) => write!(f, "An ISCSI error occured."),
            Error::Zbus(err) => err.fmt(f),
        }
    }
}

impl From<zbus::Error> for Error {
    fn from(value: zbus::Error) -> Self {
        let zbus::Error::MethodError(ref name, ref _msg, ref _info) = value else {
            return Error::Zbus(value);
        };

        match name.as_str() {
            "org.freedesktop.UDisks2.Error.Failed" => Error::Failed,
            "org.freedesktop.UDisks2.Error.Cancelled" => Error::Cancelled,
            "org.freedesktop.UDisks2.Error.AlreadyCancelled" => Error::AlreadyCancelled,
            "org.freedesktop.UDisks2.Error.NotAuthorized" => Error::NotAuthorized,
            "org.freedesktop.UDisks2.Error.NotAuthorizedCanObtain" => Error::NotAuthorizedCanObtain,
            "org.freedesktop.UDisks2.Error.NotAuthorizedDismissed" => Error::NotAuthorizedDismissed,
            "org.freedesktop.UDisks2.Error.AlreadyMounted" => Error::AlreadyMounted,
            "org.freedesktop.UDisks2.Error.NotMounted" => Error::NotMounted,
            "org.freedesktop.UDisks2.Error.OptionNotPermitted" => Error::OptionNotPermitted,
            "org.freedesktop.UDisks2.Error.MountedByOtherUser" => Error::MountedByOtherUser,
            "org.freedesktop.UDisks2.Error.AlreadyUnmounting" => Error::AlreadyUnmounting,
            "org.freedesktop.UDisks2.Error.NotSupported" => Error::NotSupported,
            "org.freedesktop.UDisks2.Error.Timedout" => Error::TimedOut,
            "org.freedesktop.UDisks2.Error.WouldWakeup" => Error::WouldWakeup,
            "org.freedesktop.UDisks2.Error.DeviceBusy" => Error::DeviceBusy,
            "org.freedesktop.UDisks2.Error.ISCSI.DaemonTransportFailed" => {
                Error::Iscsi(Iscsi::DaemonTransportFailed)
            }
            "org.freedesktop.UDisks2.Error.ISCSI.HostNotFound" => Error::Iscsi(Iscsi::HostNotFound),
            "org.freedesktop.UDisks2.Error.ISCSI.IDMB" => Error::Iscsi(Iscsi::Idmb),
            "org.freedesktop.UDisks2.Error.ISCSI.LoginFailed" => Error::Iscsi(Iscsi::LoginFailed),
            "org.freedesktop.UDisks2.Error.ISCSI.LoginAuthFailed" => {
                Error::Iscsi(Iscsi::LoginAuthFailed)
            }
            "org.freedesktop.UDisks2.Error.ISCSI.LoginFatal" => Error::Iscsi(Iscsi::LoginFatal),
            "org.freedesktop.UDisks2.Error.ISCSI.LogoutFailed" => Error::Iscsi(Iscsi::LogoutFailed),
            "org.freedesktop.UDisks2.Error.ISCSI.NoFirmware" => Error::Iscsi(Iscsi::NoFirmware),
            "org.freedesktop.UDisks2.Error.ISCSI.NoObjectsFound" => {
                Error::Iscsi(Iscsi::NoObjectsFound)
            }
            "org.freedesktop.UDisks2.Error.ISCSI.NotConnected" => Error::Iscsi(Iscsi::NotConnected),
            "org.freedesktop.UDisks2.Error.ISCSI.TransportFailed" => {
                Error::Iscsi(Iscsi::TransportFailed)
            }
            "org.freedesktop.UDisks2.Error.ISCSI.UnknownDiscoveryType" => {
                Error::Iscsi(Iscsi::UnknownDiscoveryType)
            }
            _ => Error::Zbus(value),
        }
    }
}

impl From<zbus::fdo::Error> for Error {
    fn from(value: zbus::fdo::Error) -> Self {
        Error::Zbus(value.into())
    }
}
impl From<zbus::zvariant::Error> for Error {
    fn from(value: zbus::zvariant::Error) -> Self {
        Error::Zbus(value.into())
    }
}

impl From<Infallible> for Error {
    fn from(i: Infallible) -> Self {
        match i {}
    }
}
