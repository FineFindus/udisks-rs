# Udisks

An unofficial Rust client crate for [`udisks2`](https://github.com/storaged-project/udisks).
Udisks allows accessing and manipulating disks, storages devices and similar technologies. 

## Version
Based on https://github.com/storaged-project/udisks/commit/4b1250cdf5897391e449ca0ad3836598c3b00dad for the
client and https://github.com/storaged-project/udisks/commit/3e499ea0248ee73043aedab834f32501019830a8 for the
generated interfaces.

## Example

```rust
// we use tokio in this example, but you can use any runtime
#[tokio::main]
async fn main() -> udisks2::Result<()> {
    let client = udisks2::Client::new().await?;
    let object = client
        .object("/org/freedesktop/UDisks2/block_devices/sda")
        .expect("No sda device found");
    let block = object.block().await?;
    let drive = client.drive_for_block(&block).await?;
    println!(
        "Size: {}",
        client.size_for_display(drive.size().await?, true, true)
    );
    Ok(())
}
```

## Documentation

While this crate provides documentation for the handwritten code, the generated types may be lacking. In this case it is recommended to use the official [documentation](https://storaged.org/doc/udisks2-api/latest/).

### Internationalization

This crate uses the same localization as `UDisks2`, which uses gettext. If the locale is left unset, English will be used.
