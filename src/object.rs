use zbus::{fdo::ObjectManagerProxy, zvariant::OwnedObjectPath};

use crate::nvme;
use crate::{
    ata, block, drive, encrypted, filesystem, job, mdraid, partition, partitiontable, r#loop,
    swapspace,
};

/// Utility struct for easily accessing interfaces.
pub struct Object {
    connection: zbus::Connection,
    path: OwnedObjectPath,
    object_manager: ObjectManagerProxy<'static>,
}

macro_rules! get_interface {
    ($($name:ident, $type:ty, $key:literal);+) => {
        $(
        #[doc = "Returns the `"]
        #[doc = $key]
        #[doc = "` interface."]
        ///
        /// # Errors
        /// Returns [zbus::Error::InterfaceNotFound] if the interface could not be acquired.
        pub async fn $name(&self) -> zbus::Result<$type> {
            let objects = self.object_manager.get_managed_objects().await?;
            let interfaces = objects
                .get(&self.path)
                .ok_or(zbus::Error::InterfaceNotFound)?;
            if !interfaces.contains_key($key) {
                return Err(zbus::Error::InterfaceNotFound);
            }
            <$type>::builder(&self.connection)
                .path(self.path.clone())?
                .build()
                .await
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

    get_interface!(
        block, block::BlockProxy<'_>, "org.freedesktop.UDisks2.Block";
        drive, drive::DriveProxy<'_>, "org.freedesktop.UDisks2.Drive";
        partition_table, partitiontable::PartitionTableProxy<'_>, "org.freedesktop.UDisks2.PartitionTable";
        drive_ata, ata::AtaProxy<'_>, "org.freedesktop.UDisks2.Drive.Ata";
        filesystem, filesystem::FilesystemProxy<'_>, "org.freedesktop.UDisks2.Filesystem";
        job, job::JobProxy<'_>, "org.freedesktop.UDisks2.Job";
        swapspace, swapspace::SwapspaceProxy<'_>, "org.freedesktop.UDisks2.Swapspace";
        encrypted, encrypted::EncryptedProxy<'_>, "org.freedesktop.UDisks2.Encrypted";
        r#loop, r#loop::LoopProxy<'_>, "org.freedesktop.UDisks2.Loop";
        manager_nvme, nvme::NVMeProxy<'_>, "org.freedesktop.UDisks2.Manager.Nvme";
        partition, partition::PartitionProxy<'_>, "org.freedesktop.UDisks2.Partition";
        partitiontable, partitiontable::PartitionTableProxy<'_>, "org.freedesktop.UDisks2.PartitionTable";
        mdraid, mdraid::MDRaidProxy<'_>, "org.freedesktop.UDisks2.Mdraid";
        nvme_controller, nvme::controller::ControllerProxy<'_>, "org.freedesktop.UDisks2.Nvme.Controller";
        nvme_namespace, nvme::namespace::NamespaceProxy<'_>, "org.freedesktop.UDisks2.NVMe.Namespace";
        nvme_fabrics, nvme::fabrics::FabricsProxy<'_>, "org.freedesktop.UDisks2.Nvme.Fabrics"
    );
}
