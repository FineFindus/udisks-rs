use std::{
    collections::HashMap,
    fs,
    os::{linux::fs::MetadataExt, unix::fs::FileTypeExt},
    process::ExitCode,
};

// https://github.com/storaged-project/udisks/blob/master/tools/umount-udisks.c

#[tokio::main]
async fn main() -> ExitCode {
    let mut args = std::env::args();
    let bin_name = args.next().unwrap();

    let Some(path) = args.next() else {
        eprintln!(
            "{}: this program is only supposed to be invoked by umount(8).",
            bin_name
        );
        return ExitCode::FAILURE;
    };

    let block_device = match fs::metadata(&path) {
        Ok(data) if data.file_type().is_block_device() => data.st_rdev(),
        Ok(data) => data.st_dev(),
        Err(err) => {
            eprintln!("{}: error calling stat on {}: {}).", bin_name, path, err);
            return ExitCode::FAILURE;
        }
    };

    let Ok(client) = udisks2::Client::new().await else {
        eprintln!("Error connecting to the udisks daemon");
        return ExitCode::FAILURE;
    };

    let Some(object) = lookup_object_for_block(&client, block_device).await else {
        eprintln!(
            "Error finding object for block device {}:{}",
            major(block_device),
            minor(block_device)
        );
        return ExitCode::FAILURE;
    };

    let Ok(filesystem) = object.filesystem().await else {
        eprintln!(
            "Block device {}:{} is not a mountable filesystem",
            major(block_device),
            minor(block_device)
        );
        return ExitCode::FAILURE;
    };

    if let Err(err) = filesystem.unmount(HashMap::new()).await {
        eprintln!(
            "Error unmounting block device {}:{}: {}",
            major(block_device),
            minor(block_device),
            err
        );
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}

async fn lookup_object_for_block(
    client: &udisks2::Client,
    block_device: u64,
) -> Option<udisks2::Object> {
    for object in client
        .object_manager()
        .get_managed_objects()
        .await
        .into_iter()
        .flatten()
        .filter_map(|(object_path, _)| client.object(object_path).ok())
    {
        if let Ok(block) = object.block().await {
            if Ok(block_device) == block.device_number().await {
                return Some(object);
            }
        };
    }
    None
}
pub fn major(dev: u64) -> u32 {
    let mut major = 0;
    major |= (dev & 0x00000000000fff00) >> 8;
    major |= (dev & 0xfffff00000000000) >> 32;
    major as u32
}

pub fn minor(dev: u64) -> u32 {
    let mut minor = 0;
    minor |= (dev & 0x00000000000000ff) >> 0;
    minor |= (dev & 0x00000ffffff00000) >> 12;
    minor as u32
}
