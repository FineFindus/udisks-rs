//! D-Bus interface for long-running UDisks2 operations.
//!
//! Some operations may take a long time (hours) to complete, such as formatting
//! a block device. When such operations are initiated, a job object implementing
//! this interface may be created so the progress can be tracked by the caller
//! and other observers.
//!
//! The objects that a job affects (such as block devices or drives) can be
//! determined by looking at the [`objects`](Self::objects) property. This can be used
//! to draw a spinner in the user interface next to an icon for the drive or device.
//!
//! When a job completes, the [`completed`](Self::completed) signal is emitted.
use zbus::proxy;

use crate::error;

#[proxy(
    interface = "org.freedesktop.UDisks2.Job",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Job"
)]
pub trait Job {
    /// Cancels the job.
    ///
    /// # Errors
    ///
    /// Fails with [`error::Error::Failed`] if the job is not
    /// [`Self::cancelable`].
    fn cancel(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> error::Result<()>;

    /// Emitted when a job completes.
    //TODO: should the arguments be out arguments?
    #[zbus(signal)]
    fn completed(&self, success: bool, message: &str) -> error::Result<()>;

    /// Total number of bytes to process.
    ///
    /// If the job involves processing a known number of bytes (for example,
    /// erasing a disk), this property contains the total number of bytes to process.
    /// If not, the value is zero.
    ///
    /// The intent of this property is for user interfaces to convey information
    /// such as "123 GB of 1.0 TB completed".
    #[zbus(property)]
    fn bytes(&self) -> error::Result<u64>;

    /// Whether the job can be canceled.
    #[zbus(property)]
    fn cancelable(&self) -> error::Result<bool>;

    /// The expected point in time (micro-seconds since [`std::time::UNIX_EPOCH`])
    /// that the job will complete or 0 if unknown.
    #[zbus(property)]
    fn expected_end_time(&self) -> error::Result<u64>;

    /// Objects related to this job, if any.
    #[zbus(property)]
    fn objects(&self) -> error::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Type of operation that the job represents.
    ///
    /// # Known Operation Types
    /// * `ata-smart-selftest` - SMART self-test operation
    /// * `drive-eject` - Ejecting medium from a drive
    /// * `encrypted-unlock` - Unlocking encrypted device
    /// * `encrypted-lock` - Locking encrypted device
    /// * `encrypted-modify` - Modifying encrypted device
    /// * `encrypted-resize` - Resizing encrypted device
    /// * `swapspace-start` - Starting swapspace
    /// * `swapspace-stop` - Stopping swapspace
    /// * `swapspace-modify` - Modifying swapspace
    /// * `filesystem-mount` - Mounting a filesystem
    /// * `filesystem-unmount` - Unmounting a filesystem
    /// * `filesystem-modify` - Modifying a filesystem
    /// * `filesystem-resize` - Resizing a filesystem
    /// * `format-erase` - Erasing a device
    /// * `format-mkfs` - Creating a filesystem
    /// * `loop-setup` - Setting up a loop device
    /// * `partition-modify` - Modifying a partition
    /// * `partition-delete` - Deleting a partition
    /// * `partition-create` - Creating a partition
    /// * `cleanup` - Cleaning up devices removed without proper unmounting
    /// * `ata-secure-erase` - ATA Secure Erase
    /// * `ata-enhanced-secure-erase` - ATA Enhanced Secure Erase
    /// * `md-raid-stop` - Stopping a RAID Array
    /// * `md-raid-start` - Starting a RAID Array
    /// * `md-raid-fault-device` - Marking device in RAID Array as faulty
    /// * `md-raid-remove-device` - Removing device from RAID Array
    /// * `md-raid-create` - Creating a RAID Array
    /// * `nvme-selftest` - NVMe device self-test operation
    /// * `nvme-sanitize` - NVMe sanitize operation
    /// * `nvme-format-ns` - NVMe format namespace operation
    #[zbus(property)]
    fn operation(&self) -> error::Result<String>;

    /// How much progress has been made.
    ///
    /// Values are in the range 0.0 to 1.0.
    /// Do not use unless [`Self::progress_valid`] is `true`.
    #[zbus(property)]
    fn progress(&self) -> error::Result<f64>;

    /// Whether the progress value is valid.
    #[zbus(property)]
    fn progress_valid(&self) -> error::Result<bool>;

    /// If the job involves processing a number of bytes (for example, erasing)
    /// and the rate at which the processing takes place is known, this property
    /// contains the rate measured in bytes per second. Otherwise the value is zero.
    ///
    /// The intent of this property is for user interfaces to convey information
    /// such as "110 MB/sec".
    #[zbus(property)]
    fn rate(&self) -> error::Result<u64>;

    /// Point in time (micro-seconds since [`std::time::UNIX_EPOCH`])
    /// that the job was started.
    #[zbus(property)]
    fn start_time(&self) -> error::Result<u64>;

    /// ID of the user who started the job or 0 if started
    /// by root or not through udisks.
    #[zbus(property, name = "StartedByUID")]
    fn started_by_uid(&self) -> error::Result<u32>;
}
