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
        //TODO: use gettext
        //https://github.com/storaged-project/udisks/blob/0b3879ab1d429b8312eaad0deb1b27e5545e39c1/udisks/udisksobjectinfo.c#L252
        self.icon = Some(GIcon("drive-removable-media"));
        self.icon_symbolic = Some(GIcon("drive-removable-media_symbolic"));
        self.name = block
            .preferred_device()
            .await
            .ok()
            .and_then(|dev| String::from_utf8(dev).ok());

        let size = block.size().await;
        if let Ok(size) = size {
            let size = client.size_for_display(size, false, false);
            self.description = Some(format!("{} Block Device", size));
        } else {
            self.description = Some("Block Device".to_owned());
        }

        let mut partition_number = None;
        if let Some(partition) = partition {
            //TODO: we're expecting it here to to be fine to load,
            //but further down we handle the error???
            partition_number = partition.number().await.ok();

            // Translators: Used to describe a partition of a block device.
            //              The %u is the partition number.
            //              The %s is the description for the block device (e.g. "5 GB Block Device").
            self.description = Some(format!(
                "Partition {} of {}",
                partition_number.expect("Failed to read partition number"),
                //Safe to unwrap, we have previously set this
                self.description.as_ref().unwrap()
            ));
        }

        // Translators: String used for one-liner description of a block device.
        //              The first %s is the description of the object (e.g. "50 GB Block Device").
        //              The second %s is the special device file (e.g. "/dev/sda2").
        //TODO: C version calls preferred_device again, instead of using name, why?
        self.one_liner = Some(format!(
            "{} ({})",
            self.description.as_ref().unwrap(),
            self.name.as_ref().unwrap()
        ));

        self.sort_key = Some(format!(
            "02_block_{}_{}",
            // safe to unwrap, object apth always have at least one `/`
            self.object.object_path().split('/').last().unwrap(),
            //TODO: use asnyc closure when stable
            partition_number.unwrap_or(0)
        ))
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
