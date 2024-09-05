//! Interface to represent both hard disks and disk drives
//! (with or without removable media).
//!
//! This interface should not to be confused with the `org.freedesktop.UDisks2.Block`
//! interface that is used for low-level block devices the OS knows about.
//! For example, if `/dev/sda` and `/dev/sdb` are block devices for two paths
//! to the same drive, there will be only one `org.freedesktop.UDisks2.Drive`
//! object but two `org.freedesktop.UDisks2.Block` objects.

use std::str::FromStr;

use serde::{de::IntoDeserializer, Deserialize, Serialize};
use zbus::{
    proxy,
    zvariant::{OwnedValue, Type, Value},
};

use crate::error;

/// Rotational rate of a drive.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RotationRate {
    /// The drive is known to be rotating media but rotation rate isn't known.
    Unknown,
    /// The drive is known to be non-rotating media.
    #[default]
    NonRotating,
    /// The rotation rate in rounds per minute.
    Rotating(i32),
}

impl TryFrom<OwnedValue> for RotationRate {
    type Error = <i32 as TryFrom<OwnedValue>>::Error;

    fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
        Ok(match v.try_into()? {
            -1 => RotationRate::Unknown,
            0 => RotationRate::NonRotating,
            v => RotationRate::Rotating(v),
        })
    }
}

/// The physical kind of media a drive uses or the type of the drive.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MediaCompatibility {
    /// The device is a thumb-drive with non-removable media (e.g. a USB stick)
    Thumb,
    /// Flash Card
    Flash,
    /// CompactFlash
    FlashCf,
    /// MemoryStick
    FlashMs,
    /// SmartMedia
    FlashSm,
    /// Secure Digital
    FlashSd,
    /// Secure Digital High Capacity
    FlashSdhc,
    /// Secure Digital eXtended Capacity
    FlashSdxc,
    /// Secure Digital Input Output
    FlashSdio,
    /// Secure Digital Input Output combo card with storage and I/O functionality
    FlashSdCombo,
    /// MultiMediaCard
    FlashMmc,
    /// Floppy Disk
    Floppy,
    /// Zip Disk
    FloppyZip,
    /// Jaz Disk
    FloppyJaz,
    /// Optical Disc
    Optical,
    /// Compact Disc
    OpticalCd,
    /// Compact Disc Recordable
    OpticalCdR,
    /// Compact Disc Rewritable
    OpticalCdRw,
    /// Digital Versatile Disc
    OpticalDvd,
    /// DVD-R
    OpticalDvdR,
    /// DVD-RW
    OpticalDvdRw,
    /// DVD-RAM
    OpticalDvdRam,
    /// DVD+R
    OpticalDvdPlusR,
    /// DVD+RW
    OpticalDvdPlusRw,
    /// DVD+R Dual Layer
    OpticalDvdPlusRDl,
    /// DVD+RW Dual Layer
    OpticalDvdPlusRwDl,
    /// Blu-ray Disc
    OpticalBd,
    /// Blu-ray Recordable
    OpticalBdR,
    /// Blu-ray Rewritable
    OpticalBdRe,
    /// HD-DVD
    OpticalHddvd,
    /// HD-DVD Recordable
    OpticalHddvdR,
    /// HD-DVD Rewritable
    OpticalHddvdRw,
    /// Magneto Optical
    OpticalMo,
    /// Can read Mount Rainer media
    OpticalMrw,
    /// Can write Mount Rainer media
    OpticalMrwW,
    /// Media is unknown
    #[serde(rename(deserialize = ""))] // unknow types are blank
    Unknown,
}

impl FromStr for MediaCompatibility {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<_, Self::Err> = Self::deserialize(s.into_deserializer());
        Ok(res.unwrap_or(Self::Unknown))
    }
}

// TODO: why isn't this working just using pro macros (Type)
impl TryFrom<Value<'_>> for MediaCompatibility {
    type Error = <String as TryFrom<Value<'static>>>::Error;

    fn try_from(value: Value<'_>) -> Result<Self, Self::Error> {
        let val: String = value.downcast_ref()?;
        Ok(Self::from_str(&val).unwrap_or(Self::Unknown))
    }
}

impl TryFrom<OwnedValue> for MediaCompatibility {
    type Error = <String as TryFrom<OwnedValue>>::Error;

    fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
        Self::try_from(Into::<Value<'_>>::into(v))
    }
}

#[proxy(
    interface = "org.freedesktop.UDisks2.Drive",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Drive"
)]
trait Drive {
    /// Ejects media from the drive. This is only meaningful to do on drives with removable media.
    /// There are not a lot of guarantees associated with this method so it should only be called in response to an user action.
    ///
    /// On some hardware the media may be physically ejected while on other hardware may simply eject the disc. On some hardware it may not do anything physical but it may cause e.g. a display on the hardware to show e.g. “It is now safe to remove the device”.
    fn eject(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Arranges for the drive to be safely removed and powered off.
    /// On the OS side this includes ensuring that no process is using the drive,
    /// then requesting that in-flight buffers and caches are committed to stable storage.
    /// The exact steps for powering off the drive depends on the drive itself and the interconnect used.
    /// For drives connected through USB, the effect is that the USB device will be deconfigured followed by disabling the upstream hub port it is connected to.
    ///
    /// Note that as some physical devices contain multiple drives (for example 4-in-1 flash card reader USB devices) powering off one drive may affect other drives. Applications can examine the "SiblingId" property to determine such relationships.
    ///
    /// There are not a lot of guarantees associated with this method so it should only be called in response to an user action. Usually the effect is that the drive disappears as if it was unplugged.
    ///
    /// This method only works if the [`Self::can_power_off`] property is set to `true`.
    fn power_off(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Sets the configuration for the drive.
    /// This will store the configuration in the file-system and also apply it to the drive.
    ///
    /// See the [Self::configuration] property for details about valid values and the location of the configuration file that value will be written to.
    fn set_configuration(
        &self,
        value: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    ///  Whether the drive can be safely removed / powered off. See the [Self::power_off] function for more information.
    ///
    ///  See [udisks(8)](http://storaged.org/doc/udisks2-api/latest/udisks.8.html) for how to influence the value of this property.
    #[zbus(property)]
    fn can_power_off(&self) -> error::Result<bool>;

    /// Configuration directives that are applied to the drive
    /// when it's connected (i.e. start-up, hotplug or resume).
    //TODO: since the confi. are known, use a struct?
    #[zbus(property)]
    fn configuration(
        &self,
    ) -> error::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Physical connection bus used for the drive as seen by the user.
    /// This is typically used to draw a USB or
    /// Firewire emblem on top of an icon in an user interface.
    ///
    /// Note that this property has _nothing_ to do with the low-level
    /// command-set used (such as ATA-8) or what low-level connection bus
    /// (such as SATA, eSATA, PATA, SAS2 etc) is used.
    #[zbus(property)]
    fn connection_bus(&self) -> error::Result<String>;

    /// Whether the media can be ejected from the drive or the drive accepts an eject command to switch its state so it displays e.g. a "Safe To Remove" message to the user.
    ///
    /// Note that this is only a _guess_.
    #[zbus(property)]
    fn ejectable(&self) -> error::Result<bool>;

    /// Unique and persistent identifier for the device or blank if no identifier is available.
    /// This identifier is guaranteed to not include the slash character `/` (U+002F SOLIDUS) which means it can be used as a filename.
    ///
    /// # Examples
    /// - `ST32000542AS-6XW00W51`
    /// - `HITACHI-HTS723232A7A364-E3834563KRG2HN`
    /// - `INTEL-SSDSA2MH080G1GC-CVEM842101HD080DGN`
    #[zbus(property)]
    fn id(&self) -> error::Result<String>;

    /// Media currently in the drive or black if unknown.
    #[zbus(property)]
    fn media(&self) -> error::Result<MediaCompatibility>;

    /// If the medium is available.
    ///
    /// Will always be `true` if [`Self::media_change_detected`] is `false`
    #[zbus(property)]
    fn media_available(&self) -> error::Result<bool>;

    /// If media changes are detected.
    ///
    /// Media changes are detected on all modern disk drives through
    /// either polling or an asynchronous notification mechanism.
    /// The only known disk drives that cannot report
    /// media changes are PC floppy drives.
    #[zbus(property)]
    fn media_change_detected(&self) -> error::Result<bool>;

    /// The physical kind of media the drive uses or the type of the drive.
    /// Blank if unknown.
    #[zbus(property)]
    fn media_compatibility(&self) -> error::Result<Vec<MediaCompatibility>>;

    /// Hint whether the drive and/or its media is considered removable by the user.
    ///
    /// This includes drives with removable media (cf. the [`Self::media_removable`] property),
    /// Flash media such as SD cards and drives on hotpluggable buses such as USB or Firewire (cf. the [`Self::connection_bus`] property).
    ///
    /// Note that this is only a guess.
    #[zbus(property)]
    fn media_removable(&self) -> error::Result<bool>;

    /// Name for the model of the drive or blank if unknown.
    #[zbus(property)]
    fn model(&self) -> error::Result<String>;

    /// Whether the drive contains an optical disc.
    #[zbus(property)]
    fn optical(&self) -> error::Result<bool>;

    /// Whether the disc is blank.
    ///
    /// This is only valid if the property [Self::optical] is true`.
    #[zbus(property)]
    fn optical_blank(&self) -> error::Result<bool>;

    /// Number of audio tracks.
    ///
    /// This is only valid if the property [Self::optical] is `true`.
    #[zbus(property)]
    fn optical_num_audio_tracks(&self) -> error::Result<u32>;

    /// Number of data tracks.
    ///
    /// This is only valid if the property [Self::optical] is `true`.
    #[zbus(property)]
    fn optical_num_data_tracks(&self) -> error::Result<u32>;

    /// Number of sessions.
    ///
    /// This is only valid if the property [Self::optical] is `true`.
    #[zbus(property)]
    fn optical_num_sessions(&self) -> error::Result<u32>;

    /// Number of of tracks.
    ///
    /// This is only valid if the property [Self::optical] is `true`.
    #[zbus(property)]
    fn optical_num_tracks(&self) -> error::Result<u32>;

    /// Hint whether the drive and/or its media is considered removable by the user.
    ///
    /// This includes drives with removable media (cf. the [Self::media_removable] property),
    /// Flash media such as SD cards and drives on hotpluggable buses
    /// such as USB or Firewire (cf. the [Self::connection_bus] property).
    ///
    /// Note that this is only a guess.
    #[zbus(property)]
    fn removable(&self) -> error::Result<bool>;

    /// Firmware Revision
    ///
    /// Blank if unknown.
    #[zbus(property)]
    fn revision(&self) -> error::Result<String>;

    /// Rotational rate of the drive.
    #[zbus(property)]
    fn rotation_rate(&self) -> error::Result<RotationRate>;

    /// String identifying what seat the drive is plugged into, if any.
    #[zbus(property)]
    fn seat(&self) -> error::Result<String>;

    /// Serial number of the drive.
    ///
    /// Blank if unknown.
    #[zbus(property)]
    fn serial(&self) -> error::Result<String>;

    /// Opaque token that, if non-blank,
    /// is used to group drives that are part of the same physical device.
    #[zbus(property)]
    fn sibling_id(&self) -> error::Result<String>;

    /// Size of the drive (or the media currently in the drive).
    ///
    /// In case of NVMe this value indicates the total NVM capacity that is accessible by the NVMe controller.
    /// This is always `0` if [`Self::media_change_detected`] is `false`.
    #[zbus(property)]
    fn size(&self) -> error::Result<u64>;

    /// String that can be used for sorting drive objects.
    #[zbus(property)]
    fn sort_key(&self) -> error::Result<String>;

    /// The time the drive was first detected.
    ///
    /// This is expressed in micro-seconds since [std::time::UNIX_EPOCH].
    #[zbus(property)]
    fn time_detected(&self) -> error::Result<u64>;

    /// The earliest time media was last detected or 0 if media is not available.
    ///
    /// This is expressed in micro-seconds since [std::time::UNIX_EPOCH].
    #[zbus(property)]
    fn time_media_detected(&self) -> error::Result<u64>;

    /// Name for the vendor of the drive or blank if unknown.
    #[zbus(property)]
    fn vendor(&self) -> error::Result<String>;

    /// [World Wide Name](http://en.wikipedia.org/wiki/World_Wide_Name) of the drive or blank if unknown.
    ///
    /// In case of NVMe drives please refer to namespace-level WWN properties.
    #[zbus(property, name = "WWN")]
    fn wwn(&self) -> error::Result<String>;
}
