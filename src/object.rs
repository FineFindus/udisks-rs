use zbus::fdo::ObjectManagerProxy;
use zbus::zvariant::OwnedObjectPath;

use crate::{
    ata, block, drive, encrypted, filesystem, job, mdraid, partition, partitiontable, r#loop,
    swapspace,
};
use crate::{error, nvme};

/// Utility struct for easily accessing interfaces.
#[derive(Debug, Clone)]
pub struct Object {
    connection: zbus::Connection,
    path: OwnedObjectPath,
    object_manager: ObjectManagerProxy<'static>,
}

/// Generate functions to get the interfaces of the given paths.
///
/// # Examples
///
/// ```skip
/// # fn main() {
/// impl_get_interface!(
///  block, block::BlockProxy<'static>, "org.freedesktop.UDisks2.Block";
/// );
///
/// # }
/// ```
macro_rules! impl_get_interface {
    ($($name:ident, $type:ty, $key:literal);+) => {
        $(
        #[doc = "Returns the `"]
        #[doc = $key]
        #[doc = "` interface."]
        ///
        /// # Errors
        /// Returns [zbus::Error::InterfaceNotFound] if the interface could not be acquired.
        pub async fn $name(&self) -> error::Result<$type> {
            let objects = self.object_manager.get_managed_objects().await?;
            let interfaces = objects
                .get(&self.path)
                .ok_or(zbus::Error::InterfaceNotFound)?;
            if !interfaces.contains_key($key) {
                return Err(zbus::Error::InterfaceNotFound.into());
            }
            Ok(<$type>::builder(&self.connection)
                .path(self.path.clone())?
                .build()
                .await?)
        })+
    };
}

impl Object {
    pub(crate) fn new(
        path: OwnedObjectPath,
        object_manager: ObjectManagerProxy<'static>,
        connection: zbus::Connection,
    ) -> Self {
        Self {
            connection,
            path,
            object_manager,
        }
    }

    /// Returns the [OwnedObjectPath] used by the object.
    pub fn object_path(&self) -> &OwnedObjectPath {
        &self.path
    }

    impl_get_interface!(
        block, block::BlockProxy<'static>, "org.freedesktop.UDisks2.Block";
        drive, drive::DriveProxy<'static>, "org.freedesktop.UDisks2.Drive";
        drive_ata, ata::AtaProxy<'static>, "org.freedesktop.UDisks2.Drive.Ata";
        filesystem, filesystem::FilesystemProxy<'static>, "org.freedesktop.UDisks2.Filesystem";
        job, job::JobProxy<'static>, "org.freedesktop.UDisks2.Job";
        swapspace, swapspace::SwapspaceProxy<'static>, "org.freedesktop.UDisks2.Swapspace";
        encrypted, encrypted::EncryptedProxy<'static>, "org.freedesktop.UDisks2.Encrypted";
        r#loop, r#loop::LoopProxy<'static>, "org.freedesktop.UDisks2.Loop";
        manager_nvme, nvme::NVMeProxy<'static>, "org.freedesktop.UDisks2.Manager.Nvme";
        partition, partition::PartitionProxy<'static>, "org.freedesktop.UDisks2.Partition";
        partition_table, partitiontable::PartitionTableProxy<'static>, "org.freedesktop.UDisks2.PartitionTable";
        mdraid, mdraid::MDRaidProxy<'static>, "org.freedesktop.UDisks2.Mdraid";
        nvme_controller, nvme::controller::ControllerProxy<'static>, "org.freedesktop.UDisks2.NVMe.Controller";
        nvme_namespace, nvme::namespace::NamespaceProxy<'static>, "org.freedesktop.UDisks2.NVMe.Namespace";
        nvme_fabrics, nvme::fabrics::FabricsProxy<'static>, "org.freedesktop.UDisks2.Nvme.Fabrics"
    );
}
