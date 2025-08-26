use std::ffi::CString;

use gettextrs::{gettext, pgettext};

use crate::{
    Client, Object, block,
    drive::{self, RotationRate},
    error,
    gettext::{dpgettext, gettext_f, pgettext_f},
    r#loop,
    manager::RaidLevel,
    mdraid,
    media::{self, DriveType},
    partition,
};

/// Icon
///
/// Represents an icon that can be looked up from an icon theme.
/// An icon may have an symbolic version as well.
#[derive(Debug, Default, Clone)]
pub struct Icon {
    name: Option<String>,
    name_symbolic: Option<String>,
}

impl Icon {
    fn new(name: Option<String>, name_symbolic: Option<String>) -> Self {
        Self {
            name,
            name_symbolic,
        }
    }

    fn set_if_none(&mut self, icon: String, icon_symbolic: String) {
        self.name.get_or_insert(icon);
        self.name_symbolic.get_or_insert(icon_symbolic);
    }

    /// Name of the icon.
    ///
    /// If the [`Object`] has no associated icon, None is returned.
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Name of the symbolic icon.
    ///
    /// If the [`Object`] has no associated symbolic icon, None is returned.
    pub fn name_symbolic(&self) -> Option<&String> {
        self.name_symbolic.as_ref()
    }
}

/// Detailed information about the D-Bus interfaces (such as [`block::BlockProxy`] and [`drive::DriveProxy`])
/// on a [`Object`] that is suitable to display in an user interface.
#[derive(Debug, Clone)]
pub struct ObjectInfo<'a> {
    /// The [`Object`] that the info is about
    pub object: &'a Object,

    /// Name of the object
    pub name: Option<String>,

    /// Description of the object
    pub description: Option<String>,

    /// Icon associated with the object
    ///
    /// The returned icon may be influenced by [`block::BlockProxy::hint_name()`].
    pub icon: Icon,

    /// Description of media associated with the object
    pub media_description: Option<String>,

    /// Icon associated with media
    ///
    /// The returned icon may be influenced by [`block::BlockProxy::hint_name()`].
    pub media_icon: Icon,

    /// Single-line description
    ///
    /// A single line string, containing enough detail to be used as a comprehensive
    /// representation of the `object`. For instance, in the case of block devices
    /// or drives, it includes critical information like the device's special file
    /// path, such as `/dev/sda`.
    pub one_liner: Option<String>,

    /// Sort key
    ///
    /// This can be used to sort objects.
    pub sort_key: Option<String>,
}

impl<'a> ObjectInfo<'a> {
    pub(crate) fn new(object: &'a Object) -> Self {
        Self {
            object,
            name: None,
            description: None,
            icon: Icon::default(),
            media_description: None,
            media_icon: Icon::default(),
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
        self.icon = Icon::new(
            Some("drive-removable-media".to_owned()),
            Some("drive-removable-media-symbolic".to_owned()),
        );
        self.name = block
            .preferred_device()
            .await
            .ok()
            .and_then(|dev| CString::from_vec_with_nul(dev).ok())
            .and_then(|dev| dev.to_str().map(|p| p.to_string()).ok());

        let size = block.size().await;
        if let Ok(size) = size {
            let size = client.size_for_display(size, false, false);
            self.description = Some(gettext_f("{} Block Device", [size]));
        } else {
            self.description = Some(gettext("Block Device"));
        }

        let mut partition_number = None;
        if let Some(partition) = partition {
            //TODO: we're expecting it here to to be fine to load,
            //but further down we handle the error???
            partition_number = partition.number().await.ok();

            // Translators: Used to describe a partition of a block device.
            //              The %u is the partition number.
            //              The %s is the description for the block device (e.g. "5 GB Block Device").
            //TODO: pgettext_f does not support non-string C placeholders, so we replace it
            //manually to maintain compatibilty
            self.description = Some(
                pgettext_f(
                    "part-block",
                    "Partition %u of {}",
                    [
                        //Safe to unwrap, we have previously set this
                        self.description.as_ref().unwrap(),
                    ],
                )
                .replace(
                    "%u",
                    &partition_number
                        .expect("Failed to read partition number")
                        .to_string(),
                ),
            );
        }

        // Translators: String used for one-liner description of a block device.
        //              The first %s is the description of the object (e.g. "50 GB Block Device").
        //              The second %s is the special device file (e.g. "/dev/sda2").
        //TODO: C version calls preferred_device again, instead of using name, why?
        self.one_liner = Some(pgettext_f(
            "one-liner-block",
            "{} ({})",
            [
                self.description.as_ref().unwrap(),
                self.name.as_ref().unwrap(),
            ],
        ));

        self.sort_key = Some(format!(
            "02_block_{}_{}",
            // safe to unwrap, object path always have at least one `/`
            self.object.object_path().split('/').next_back().unwrap(),
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
        self.icon = Icon::new(
            Some("drive-removable-media".to_owned()),
            Some("drive-removable-media-symbolic".to_owned()),
        );
        self.name = loop_proxy
            .backing_file()
            .await
            .ok()
            .and_then(|dev| CString::from_vec_with_nul(dev).ok())
            .and_then(|dev| dev.to_str().map(|p| p.to_string()).ok());

        let size = block.size().await;
        if let Ok(size) = size {
            let size = client.size_for_display(size, false, false);
            self.description = Some(gettext_f("{} Loop Device", [size]));
        } else {
            self.description = Some(gettext("Loop Device"));
        }

        let mut partition_number = None;
        if let Some(partition) = partition {
            //TODO: we're expecting it here to to be fine to load,
            //but further down we handle the error???
            partition_number = partition.number().await.ok();

            // Translators: Used to describe a partition of a loop device.
            //              The %u is the partition number.
            //              The %s is the description for the block device (e.g. "5 GB Loop Device").
            //TODO: pgettext_f does not support non-string C placeholders, so we replace it
            //manually to maintain compatibilty
            self.description = Some(
                pgettext_f(
                    "part-loop",
                    "Partition %u of {}",
                    [
                        //Safe to unwrap, we have previously set this
                        self.description.as_ref().unwrap(),
                    ],
                )
                .replace(
                    "%u",
                    &partition_number
                        .expect("Failed to read partition number")
                        .to_string(),
                ),
            );
        }

        // Translators: String used for one-liner description of a loop device.
        //              The first %s is the description of the object (e.g. "2 GB Loop Device").
        //              The second %s is the name of the backing file (e.g. "/home/davidz/file.iso").
        //              The third %s is the special device file (e.g. "/dev/loop2").
        self.one_liner = Some(pgettext_f(
            "one-liner-loop",
            "{} — {} ({})",
            [
                self.description.as_ref().unwrap(),
                //safe to unwrap, has been set previously
                self.name.as_ref().unwrap(),
                &block
                    .preferred_device()
                    .await
                    .ok()
                    .and_then(|dev| CString::from_vec_with_nul(dev).ok())
                    .and_then(|dev| dev.to_str().map(|p| p.to_string()).ok())
                    .unwrap_or_default(),
            ],
        ));

        self.sort_key = Some(format!(
            "03_loop_{}_{}",
            // safe to unwrap, object path always have at least one `/`
            self.object.object_path().split('/').next_back().unwrap(),
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
        self.name = Some(
            name.split(':')
                .next_back()
                .unwrap_or_else(|| &name)
                .to_string(),
        );
        self.icon = Icon::new(
            Some("drive-multidisk".to_owned()),
            Some("drive-multidisk-symbolic".to_owned()),
        );

        let level = mdraid.level().await;
        let size = mdraid.size().await;
        if let Ok(size) = size {
            let size = client.size_for_display(size, false, false);
            // Translators: Used to format the description for a RAID array.
            //              The first %s is the size (e.g. '42.0 GB').
            //              The second %s is the level (e.g. 'RAID-5 Array').
            self.description = Some(pgettext_f(
                "mdraid-desc",
                "{} {}",
                [size, self.format_level(level)],
            ));
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
            //TODO: pgettext_f does not support non-string C placeholders, so we replace it
            //manually to maintain compatibilty
            self.description = Some(
                pgettext_f(
                    "part-raid",
                    "Partition %u of {}",
                    [
                        &partition_number
                            .expect("Failed to read partition number")
                            .to_string(),
                        //Safe to unwrap, we have previously set this
                        self.description.as_ref().unwrap(),
                    ],
                )
                .replace(
                    "%u",
                    &partition_number
                        .expect("Failed to read partition number")
                        .to_string(),
                ),
            );
        }

        let block = client.block_for_mdraid(&mdraid).await;
        if self.name.as_deref().is_some_and(|name| !name.is_empty()) {
            if let Some(block) = block {
                let preferred_device = block
                    .preferred_device()
                    .await
                    .ok()
                    .and_then(|dev| CString::from_vec_with_nul(dev).ok())
                    .and_then(|dev| dev.to_str().map(|p| p.to_string()).ok())
                    .expect("Failed to get preferred device");

                // Translators: String used for one-liner description of running RAID array.
                //              The first %s is the array name (e.g. "AlphaGo").
                //              The second %s is the size and level (e.g. "2 TB RAID-5").
                //              The third %s is the special device file (e.g. "/dev/sda").
                self.one_liner = Some(pgettext_f(
                    "one-liner-mdraid-running",
                    "{} — {} ({})",
                    [
                        self.name.as_deref().unwrap(),
                        self.description.as_deref().unwrap_or_default(),
                        &preferred_device,
                    ],
                ));
            } else {
                // Translators: String used for one-liner description of non-running RAID array.
                //              The first %s is the array name (e.g. "AlphaGo").
                //              The second %s is the size and level (e.g. "2 TB RAID-5").
                self.one_liner = Some(pgettext_f(
                    "one-liner-mdraid-not-running",
                    "{} — {}",
                    [
                        self.name.as_deref().unwrap_or_default(),
                        self.description.as_deref().unwrap_or_default(),
                    ],
                ));
            }
        } else if let Some(block) = block {
            let preferred_device = block
                .preferred_device()
                .await
                .ok()
                .and_then(|dev| CString::from_vec_with_nul(dev).ok())
                .and_then(|dev| dev.to_str().map(|p| p.to_string()).ok())
                .expect("Failed to get preferred device");

            // Translators: String used for one-liner description of running RAID array.
            //              The first %s is the array name (e.g. "AlphaGo").
            //              The second %s is the size and level (e.g. "2 TB RAID-5").
            //              The third %s is the special device file (e.g. "/dev/sda").
            self.one_liner = Some(pgettext_f(
                "one-liner-mdraid-no-name-running",
                "{} — {}",
                [
                    self.description.as_deref().unwrap_or_default(),
                    &preferred_device,
                ],
            ));
        } else {
            // Translators: String used for one-liner description of non-running RAID array.
            //              The first %s is the array name (e.g. "AlphaGo").
            //              The second %s is the size and level (e.g. "2 TB RAID-5").
            self.one_liner = Some(pgettext_f(
                "one-liner-mdraid-no-name-not-running",
                "{}",
                [self.description.as_deref().unwrap_or_default()],
            ));
        }

        self.sort_key = Some(format!(
            "01_mdraid_{}_{}",
            mdraid.uuid().await.expect("Failed to get mdraid uuid"),
            //TODO: use async closure when stable
            partition_number.unwrap_or(0)
        ));
    }

    pub(crate) async fn info_for_drive(
        &mut self,
        client: &Client,
        drive: &drive::DriveProxy<'_>,
        partition: Option<partition::PartitionProxy<'_>>,
    ) {
        let vendor = drive.vendor().await.unwrap_or_default();
        // "%vendor $model"
        self.name = Some(format!(
            "{}{}{}",
            vendor,
            if vendor.is_empty() { "" } else { " " },
            drive.model().await.unwrap_or_default()
        ));

        let media_removable = drive.media_removable().await.unwrap_or_default();
        let media_available = drive.media_available().await.unwrap_or_default();
        let media = drive.media().await.unwrap();
        let media_compat = drive.media_compatibility().await.unwrap_or_default();

        let mut desc = String::new();
        let mut desc_type = None;
        for media_data in media::MEDIA_DATA {
            if media_compat.contains(&media_data.id) {
                self.icon.set_if_none(
                    media_data.drive_icon.to_owned(),
                    media_data.drive_icon_symbolic.to_owned(),
                );
                if !desc.contains(media_data.media_family) {
                    if !desc.is_empty() {
                        desc.push('/');
                    }
                    desc.push_str(&pgettext("media-type", media_data.media_family));
                }
                desc_type = Some(media_data.media_type);
            }

            if media_removable && media_available {
                //media
                if media == media_data.id {
                    if self.media_description.is_none() {
                        self.media_description = Some(match media_data.media_type {
                            media::DriveType::Drive => {
                                //Translators: Used to describe drive without removable media. The %s is the type, e.g. 'Thumb'
                                pgettext_f(
                                    "drive-with-fixed-media",
                                    "{} Drive",
                                    [dpgettext("media-type", media_data.media_name)],
                                )
                            }
                            media::DriveType::Disk => {
                                //Translators: Used to describe generic media. The %s is the type, e.g. 'Zip' or 'Floppy'
                                pgettext_f(
                                    "drive-with-generic-media",
                                    "{} Disk",
                                    [dpgettext("media-type", media_data.media_name)],
                                )
                            }
                            media::DriveType::Card => {
                                //Translators: Used to describe flash media. The %s is the type, e.g. 'SD' or 'CompactFlash'
                                pgettext_f(
                                    "flash-media",
                                    "{} Card",
                                    [dpgettext("media-type", media_data.media_name)],
                                )
                            }
                            media::DriveType::Disc => {
                                //Translators: Used to describe optical discs. The %s is the type, e.g. 'CD-R' or 'DVD-ROM'
                                pgettext_f(
                                    "optical-media",
                                    "{} Disc",
                                    [dpgettext("media-type", media_data.media_name)],
                                )
                            }
                        });
                    }

                    self.media_icon.set_if_none(
                        media_data.media_icon.to_owned(),
                        media_data.media_icon_symbolic.to_owned(),
                    );
                }
            }
        }

        let size = drive
            .size()
            .await
            .ok()
            .map(|size| client.size_for_display(size, false, false));
        let rotation_rate = drive.rotation_rate().await.unwrap_or_default();
        self.description = Some(match desc_type {
            None => {
                if media_removable {
                    if let Some(size) = size {
                        // Translators: Used to describe a drive. The %s is the size, e.g. '20 GB'
                        pgettext_f("drive-with-size", "{} Drive", [size])
                    } else {
                        //Translators: Used to describe a drive we know very little about (removable media or size not known)
                        pgettext("generic-drive", "Drive")
                    }
                } else if rotation_rate == RotationRate::NonRotating {
                    if let Some(size) = size {
                        // Translators: Used to describe a non-rotating drive (rotation rate either unknown
                        // or it's a solid-state drive). The %s is the size, e.g. '20 GB'.
                        pgettext_f("disk-non-rotational", "{} Disk", [size])
                    } else {
                        // Translators: Used to describe a non-rotating drive (rotation rate either unknown
                        // or it's a solid-state drive). The drive is either using removable media or its
                        // size not known.
                        pgettext("disk-non-rotational", "Disk")
                    }
                } else if let Some(size) = size {
                    // Translators: Used to describe a hard-disk drive (HDD). The %s is the size, e.g. '20 GB'.
                    pgettext_f("disk-hdd", "{} Hard Disk", [size])
                } else {
                    // Translators: Used to describe a hard-disk drive (HDD) (removable media or size not known)
                    pgettext("disk-hdd", "Hard Disk")
                }
            }
            Some(DriveType::Card) => {
                // Translators: Used to describe a card reader. The %s is the card type e.g. 'CompactFlash'.
                pgettext_f("drive-card-reader", "{} Card Reader", [desc])
            }
            Some(DriveType::Drive) | Some(DriveType::Disk) | Some(DriveType::Disc) => {
                if size.as_ref().is_some_and(|_| !media_removable) {
                    // Translators: Used to describe drive. The first %s is the size e.g. '20 GB' and the
                    // second %s is the drive type e.g. 'Thumb'.
                    pgettext_f(
                        "drive-with-size-and-type",
                        "{} {} Drive",
                        [size.unwrap(), desc],
                    )
                } else {
                    //Translators: Used to describe drive. The first %s is the drive type e.g. 'Thumb'.
                    pgettext_f("drive-with-type", "{} Drive", [desc])
                }
            }
        });

        let hyphenated_connection_bus = drive
            .connection_bus()
            .await
            .ok()
            .filter(|bus| !bus.is_empty())
            .map(|bus| format!("-{}", bus))
            .unwrap_or_default();

        //fallback for icon
        let icon_fallback = if media_removable {
            format!("drive-removable-media{}", hyphenated_connection_bus)
        } else if rotation_rate == RotationRate::NonRotating {
            format!("drive-harddisk-solidstate{}", hyphenated_connection_bus)
        } else {
            format!("drive-harddisk{}", hyphenated_connection_bus)
        };

        let icon_symbolic_fallback = if media_removable {
            format!(
                "drive-removable-media{}-symbolic",
                hyphenated_connection_bus
            )
        } else if rotation_rate == RotationRate::NonRotating {
            format!(
                "drive-harddisk-solidstate{}-symbolic",
                hyphenated_connection_bus
            )
        } else {
            format!("drive-harddisk{}-symbolic", hyphenated_connection_bus)
        };
        self.icon.set_if_none(icon_fallback, icon_symbolic_fallback);

        //fallback for media_icon
        if media_available {
            let media_icon_fallback = if media_removable {
                format!("drive-removable-media{}", hyphenated_connection_bus)
            } else if rotation_rate == RotationRate::NonRotating {
                format!("drive-harddisk-solidstate{}", hyphenated_connection_bus)
            } else {
                format!("drive-harddisk{}", hyphenated_connection_bus)
            };

            let media_icon_symbolic_fallback = if media_removable {
                format!(
                    "drive-removable-media{}-symbolic",
                    hyphenated_connection_bus
                )
            } else if rotation_rate == RotationRate::NonRotating {
                format!(
                    "drive-harddisk-solidstate{}-symbolic",
                    hyphenated_connection_bus
                )
            } else {
                format!("drive-harddisk{}-symbolic", hyphenated_connection_bus)
            };

            self.media_icon
                .set_if_none(media_icon_fallback, media_icon_symbolic_fallback);
        }

        //TODO: refactor
        //prepend a qualifier to the media description, based on the disc state
        if drive.optical_blank().await.unwrap_or_default() {
            // Translators: String used for a blank disc. The %s is the disc type e.g. "CD-RW Disc"
            self.media_description = Some(pgettext_f(
                "optical-media",
                "Blank {}",
                [self.media_description.as_deref().unwrap_or_default()],
            ));
        } else if drive
            .optical_num_audio_tracks()
            .await
            .is_ok_and(|tracks| tracks > 0)
            && drive
                .optical_num_data_tracks()
                .await
                .is_ok_and(|tracks| tracks > 0)
        {
            // Translators: String used for a mixed disc. The %s is the disc type e.g. "CD-ROM Disc"
            self.media_description = Some(pgettext_f(
                "optical-media",
                "Mixed {}",
                [self.media_description.as_deref().unwrap_or_default()],
            ));
        } else if drive
            .optical_num_audio_tracks()
            .await
            .is_ok_and(|tracks| tracks > 0)
            && drive
                .optical_num_data_tracks()
                .await
                .is_ok_and(|tracks| tracks == 0)
        {
            // Translators: String used for an audio disc. The %s is the disc type e.g. "CD-ROM Disc"
            self.media_description = Some(pgettext_f(
                "optical-media",
                "Audio {}",
                [self.media_description.as_deref().unwrap_or_default()],
            ));
        }

        // Apply UDISKS_NAME, UDISKS_ICON_NAME, UDISKS_SYMBOLIC_ICON_NAME hints, if available
        let block = client.block_for_drive(drive, true).await;
        if let Some(ref block) = block {
            if let Ok(hint) = block.hint_name().await {
                if !hint.is_empty() {
                    self.description = Some(hint.clone());
                    self.media_description = Some(hint);
                }
            }
            if let Ok(hint_icon) = block.hint_icon_name().await {
                if !hint_icon.is_empty() {
                    self.icon.name = Some(hint_icon.clone());
                    self.media_icon.name = Some(hint_icon);
                }
            }
            if let Ok(hint_icon_symbolic) = block.hint_symbolic_icon_name().await {
                if !hint_icon_symbolic.is_empty() {
                    self.icon.name_symbolic = Some(hint_icon_symbolic.clone());
                    self.media_icon.name_symbolic = Some(hint_icon_symbolic);
                }
            }
        }

        let mut block_for_partition = None;
        if let Some(ref partition) = partition {
            // safe to unwrap as the table's object path does not need to be converted
            let object = client.object(partition.inner().path().clone()).unwrap();
            block_for_partition = object.block().await.ok();
        }
        block_for_partition = block_for_partition.or_else(|| block.clone());

        if let Some(partition) = partition {
            // Translators: Used to describe a partition of a drive.
            //                  The %u is the partition number.
            //                  The %s is the description for the drive (e.g. "2 GB Thumb Drive").
            //TODO: pgettext_f does not support non-string C placeholders, so we replace it
            //manually to maintain compatibilty
            self.description = Some(
                pgettext_f(
                    "part-drive",
                    "Partition %u of {}",
                    [self.description.as_deref().unwrap_or_default()],
                )
                .replace(
                    "%u",
                    &partition.number().await.unwrap_or_default().to_string(),
                ),
            )
        }

        //calculate and set one-liner
        if let Some(block) = block {
            if let Ok(drive_revision) = drive.revision().await {
                // Translators: String used for one-liner description of drive.
                //  The first %s is the description of the object (e.g. "80 GB Disk" or "Partition 2 of 2 GB Thumb Drive").
                //  The second %s is the name of the object (e.g. "INTEL SSDSA2MH080G1GC").
                //  The third %s is the fw revision (e.g "45ABX21").
                //  The fourth %s is the special device file (e.g. "/dev/sda").
                self.one_liner = Some(pgettext_f(
                    "one-liner-drive",
                    "{} — {} [{}] ({})",
                    [
                        self.description.as_deref().unwrap_or_default(),
                        self.name.as_deref().unwrap_or_default(),
                        &drive_revision,
                        &block
                            .preferred_device()
                            .await
                            .ok()
                            .and_then(|dev| CString::from_vec_with_nul(dev).ok())
                            .and_then(|dev| dev.to_str().map(|p| p.to_string()).ok())
                            .unwrap_or_default(),
                    ],
                ));
            } else {
                // Translators: String used for one-liner description of drive w/o known fw revision.
                //    The first %s is the description of the object (e.g. "80 GB Disk").
                //    The second %s is the name of the object (e.g. "INTEL SSDSA2MH080G1GC").
                //    The third %s is the special device file (e.g. "/dev/sda").
                self.one_liner = Some(pgettext_f(
                    "one-liner-drive",
                    "{} — {} ({})",
                    [
                        self.description.as_deref().unwrap_or_default(),
                        self.name.as_deref().unwrap_or_default(),
                        //safe to unwrap has been set before if it was none
                        &block_for_partition
                            .unwrap()
                            .preferred_device()
                            .await
                            .ok()
                            .and_then(|dev| CString::from_vec_with_nul(dev).ok())
                            .and_then(|dev| dev.to_str().map(|p| p.to_string()).ok())
                            .unwrap_or_default(),
                    ],
                ));
            }
        }

        self.sort_key = Some(format!(
            "00_drive_{}",
            drive.sort_key().await.unwrap_or_default(),
        ));
    }

    fn format_level(&self, level: error::Result<RaidLevel>) -> String {
        pgettext(
            "mdraid-desc",
            match level {
                Ok(RaidLevel::Raid0) => "RAID-0 Array",
                Ok(RaidLevel::Raid1) => "RAID-1 Array",
                Ok(RaidLevel::Raid4) => "RAID-4 Array",
                Ok(RaidLevel::Raid5) => "RAID-5 Array",
                Ok(RaidLevel::Raid6) => "RAID-6 Array",
                Ok(RaidLevel::Raid10) => "RAID-10 Array",
                _ => "RAID Array",
            },
        )
    }
}
