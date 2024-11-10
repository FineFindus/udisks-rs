#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "linux"))]
compile_error!("UDisks only supports Linux");

use std::collections::HashMap;

//re-eport zbus
pub use zbus;

pub mod ata;
pub mod block;
mod client;
pub mod drive;
pub mod encrypted;
mod error;
pub mod filesystem;
pub(crate) mod gettext;
mod id;
pub mod job;
pub mod r#loop;
pub mod manager;
pub mod mdraid;
mod media;
pub mod nvme;
mod object;
mod object_info;
mod partition_subtypes;
pub mod partition_types;
pub use object::Object;
pub use object_info::ObjectInfo;
pub mod partition;
pub mod partitiontable;
pub mod swapspace;
pub use client::Client;
pub use error::{Error, Iscsi, Result};

/// Standard Options.
///
/// Many functions inlude a parameter `options`, which includes the following options:
/// - `no_user_auth_interaction` if set to `true`, no user interaction will happen,
/// when checking if the called function is authorized
pub fn standard_options(
    no_user_auth_interaction: bool,
) -> HashMap<&'static str, zbus::zvariant::Value<'static>> {
    HashMap::from([("auth.no_user_interaction", no_user_auth_interaction.into())])
}
