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
    ( $name:ident, $type:ty, $key:literal) => {
        #[doc = "Retruns the `"]
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
        }
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
        block,
        block::BlockProxy<'_>,
        "org.freedesktop.UDisks2.Block"
    );

    get_interface!(
        drive,
        drive::DriveProxy<'_>,
        "org.freedesktop.UDisks2.Drive"
    );

    get_interface!(
        partition_table,
        partitiontable::PartitionTableProxy<'_>,
        "org.freedesktop.UDisks2.PartitionTable"
    );

    get_interface!(
        drive_ata,
        ata::AtaProxy<'_>,
        "org.freedesktop.UDisks2.Drive.Ata"
    );

    get_interface!(
        filesystem,
        filesystem::FilesystemProxy<'_>,
        "org.freedesktop.UDisks2.Filesystem"
    );

    get_interface!(job, job::JobProxy<'_>, "org.freedesktop.UDisks2.Job");

    get_interface!(
        swapspace,
        swapspace::SwapspaceProxy<'_>,
        "org.freedesktop.UDisks2.Swapspace"
    );

    get_interface!(
        encrypted,
        encrypted::EncryptedProxy<'_>,
        "org.freedesktop.UDisks2.Encrypted"
    );

    get_interface!(
        r#loop,
        r#loop::LoopProxy<'_>,
        "org.freedesktop.UDisks2.Loop"
    );

    get_interface!(
        manager_nvme,
        crate::nvme::NVMeProxy<'_>,
        "org.freedesktop.UDisks2.Manager.Nvme"
    );

    get_interface!(
        partition,
        partition::PartitionProxy<'_>,
        "org.freedesktop.UDisks2.Partition"
    );

    get_interface!(
        partitiontable,
        partitiontable::PartitionTableProxy<'_>,
        "org.freedesktop.UDisks2.PartitionTable"
    );

    get_interface!(
        mdraid,
        mdraid::MDRaidProxy<'_>,
        "org.freedesktop.UDisks2.Mdraid"
    );

    get_interface!(
        nvme_controller,
        nvme::controller::ControllerProxy<'_>,
        "org.freedesktop.UDisks2.Nvme.Controller"
    );

    get_interface!(
        nvme_namespace,
        nvme::namespace::NamespaceProxy<'_>,
        "org.freedesktop.UDisks2.NVMe.Namespace"
    );

    get_interface!(
        nvme_fabrics,
        nvme::fabrics::FabricsProxy<'_>,
        "org.freedesktop.UDisks2.Nvme.Fabrics"
    );
}
