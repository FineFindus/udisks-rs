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
            // safe to unwrap, object path always have at least one `/`
            self.object.object_path().split('/').last().unwrap(),
            //TODO: use async closure when stable
            partition_number.unwrap_or(0)
        ))
    }

    pub(crate) async fn info_for_loop(
        &mut self,
        client: &Client,
        loop_proxy: r#loop::LoopProxy<'_>,
        block: block::BlockProxy<'_>,
        partition: Option<partition::PartitionProxy<'_>>,
    ) {
        //TODO: use gettext
        //https://github.com/storaged-project/udisks/blob/0b3879ab1d429b8312eaad0deb1b27e5545e39c1/udisks/udisksobjectinfo.c#L303
        self.icon = Some(GIcon("drive-removable-media"));
        self.icon_symbolic = Some(GIcon("drive-removable-media-symbolic"));
        self.name = loop_proxy
            .backing_file()
            .await
            .ok()
            .and_then(|file| String::from_utf8(file).ok());

        let size = block.size().await;
        if let Ok(size) = size {
            let size = client.size_for_display(size, false, false);
            self.description = Some(format!("{} Loop Device", size));
        } else {
            self.description = Some("Loop Device".to_owned());
        }

        let mut partition_number = None;
        if let Some(partition) = partition {
            //TODO: we're expecting it here to to be fine to load,
            //but further down we handle the error???
            partition_number = partition.number().await.ok();

            // Translators: Used to describe a partition of a loop device.
            //              The %u is the partition number.
            //              The %s is the description for the block device (e.g. "5 GB Loop Device").
            self.description = Some(format!(
                "Partition {} of {}",
                partition_number.expect("Failed to read partition number"),
                //Safe to unwrap, we have previously set this
                self.description.as_ref().unwrap()
            ));
        }

        // Translators: String used for one-liner description of a loop device.
        //              The first %s is the description of the object (e.g. "2 GB Loop Device").
        //              The second %s is the name of the backing file (e.g. "/home/davidz/file.iso").
        //              The third %s is the special device file (e.g. "/dev/loop2").
        self.one_liner = Some(format!(
            "{} — {} ({})",
            self.description.as_ref().unwrap(),
            //safe to unwrap, has been set previously
            self.name.as_ref().unwrap(),
            block
                .preferred_device()
                .await
                .ok()
                .and_then(|dev| String::from_utf8(dev).ok())
                .unwrap_or_default()
        ));

        self.sort_key = Some(format!(
            "03_loop_{}_{}",
            // safe to unwrap, object path always have at least one `/`
            self.object.object_path().split('/').last().unwrap(),
            //TODO: use async closure when stable
            partition_number.unwrap_or(0)
        ));
    }

    pub(crate) async fn info_for_mdraid(
        &mut self,
        client: &Client,
        mdraid: mdraid::MDRaidProxy<'_>,
        partition: Option<partition::PartitionProxy<'_>>,
    ) {
        let name = mdraid.name().await.unwrap_or_default();
        self.name = Some(name.split(":").last().unwrap_or_else(|| &name).to_string());
        self.icon = Some(GIcon("drive-multidisk"));
        self.icon_symbolic = Some(GIcon("drive-multidisk-symbolic"));

        let level = mdraid.level().await;
        let size = mdraid.size().await;
        if let Ok(size) = size {
            let size = client.size_for_display(size, false, false);
            // Translators: Used to format the description for a RAID array.
            //              The first %s is the size (e.g. '42.0 GB').
            //              The second %s is the level (e.g. 'RAID-5 Array').
            self.description = Some(format!("{} {}", size, self.format_level(level)));
        } else {
            self.description = Some(self.format_level(level));
        }

        let mut partition_number = None;
        if let Some(partition) = partition {
            //TODO: we're expecting it here to to be fine to load,
            //but further down we handle the error???
            partition_number = partition.number().await.ok();
            // Translators: Used to describe a partition of a RAID Array.
            //              The %u is the partition number.
            //              The %s is the description for the drive (e.g. "2 TB RAID-5").
            self.description = Some(format!(
                "Partition {} of {}",
                partition_number.expect("Failed to read partition number"),
                //Safe to unwrap, we have previously set this
                self.description.as_ref().unwrap()
            ));
        }

        let block = client.block_for_mdraid(&mdraid).await;
        if self.name.as_deref().is_some_and(|name| !name.is_empty()) {
            if let Some(block) = block {
                let preferred_device = block
                    .preferred_device()
                    .await
                    .ok()
                    .and_then(|dev| String::from_utf8(dev).ok())
                    .expect("Failed to get preferred device");

                // Translators: String used for one-liner description of running RAID array.
                //              The first %s is the array name (e.g. "AlphaGo").
                //              The second %s is the size and level (e.g. "2 TB RAID-5").
                //              The third %s is the special device file (e.g. "/dev/sda").
                self.one_liner = Some(format!(
                    "{} — {} ({})",
                    self.name.as_deref().unwrap(),
                    self.description.as_deref().unwrap_or_default(),
                    preferred_device,
                ));
            } else {
                // Translators: String used for one-liner description of non-running RAID array.
                //              The first %s is the array name (e.g. "AlphaGo").
                //              The second %s is the size and level (e.g. "2 TB RAID-5").
                self.one_liner = Some(format!(
                    "{} — {}",
                    self.name.as_deref().unwrap_or_default(),
                    self.description.as_deref().unwrap_or_default()
                ));
            }
        } else {
            if let Some(block) = block {
                let preferred_device = block
                    .preferred_device()
                    .await
                    .ok()
                    .and_then(|dev| String::from_utf8(dev).ok())
                    .expect("Failed to get preferred device");

                // Translators: String used for one-liner description of running RAID array.
                //              The first %s is the array name (e.g. "AlphaGo").
                //              The second %s is the size and level (e.g. "2 TB RAID-5").
                //              The third %s is the special device file (e.g. "/dev/sda").
                self.one_liner = Some(format!(
                    "{} — {}",
                    self.description.as_deref().unwrap_or_default(),
                    preferred_device,
                ));
            } else {
                // Translators: String used for one-liner description of non-running RAID array.
                //              The first %s is the array name (e.g. "AlphaGo").
                //              The second %s is the size and level (e.g. "2 TB RAID-5").
                self.one_liner = Some(self.description.as_deref().unwrap_or_default().to_string());
            }
        }

        self.sort_key = Some(format!(
            "01_mdraid_{}_{}",
            mdraid.uuid().await.expect("Failed to get mdraid uuid"),
            //TODO: use async closure when stable
            partition_number.unwrap_or(0)
        ));
    }

    pub(crate) fn info_for_drive(
        &self,
        client: &Client,
        drive: &drive::DriveProxy,
        partition: Option<partition::PartitionProxy>,
    ) {
        unimplemented!();
    }

    fn format_level(&self, level: zbus::Result<String>) -> String {
        //TODO: use gettext
        //https://github.com/storaged-project/udisks/blob/0b3879ab1d429b8312eaad0deb1b27e5545e39c1/udisks/udisksobjectinfo.c#L351    }
        match level.as_deref() {
            Ok("raid0") => "RAID-0 Array",
            Ok("raid1") => "RAID-1 Array",
            Ok("raid4") => "RAID-4 Array",
            Ok("raid5") => "RAID-5 Array",
            Ok("raid6") => "RAID-6 Array",
            Ok("raid10") => "RAID-10 Array",
            _ => "RAID Array",
        }
        .to_string()
    }
}
