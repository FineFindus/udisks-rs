use crate::{block, drive, mdraid, partition, r#loop, Client, Object};

enum DriveType {
    Unset,
    Drive,
    Disk,
    Card,
    Disc,
}

///stub
#[derive(Debug, Clone)]
//TODO: use sensible version for Rust
pub struct GIcon(&'static str);

#[derive(Debug, Clone)]
pub struct ObjectInfo {
    /// The [`Object`] that the info is about
    //TODO: use reference?
    pub object: Object,
    ///
    name: Option<String>,
    description: Option<String>,
    pub icon: Option<GIcon>,
    pub icon_symbolic: Option<GIcon>,
    media_description: Option<String>,
    media_icon: Option<String>,
    media_icon_symbolic: Option<GIcon>,
    one_liner: Option<String>,
    sort_key: Option<String>,
}

impl ObjectInfo {
    pub(crate) async fn new(object: Object) -> Self {
        Self {
            object,
            name: None,
            description: None,
            icon: None,
            icon_symbolic: None,
            media_description: None,
            media_icon: None,
            media_icon_symbolic: None,
            one_liner: None,
            sort_key: None,
        }
    }

    pub(crate) async fn info_for_block(
        &mut self,
        client: &Client,
        block: block::BlockProxy<'_>,
        partition: Option<partition::PartitionProxy<'_>>,
    ) {
        unimplemented!()
    }

    pub(crate) fn info_for_drive(
        &self,
        client: &Client,
        drive: &drive::DriveProxy,
        partition: Option<partition::PartitionProxy>,
    ) {
        unimplemented!();
    }

    pub(crate) fn info_for_mdraid(&self, mdraid: mdraid::MDRaidProxy<'_>) {
        todo!()
    }

    pub(crate) fn info_for_loop(&self, loop_proxy: r#loop::LoopProxy<'_>) {
        todo!()
    }
}
