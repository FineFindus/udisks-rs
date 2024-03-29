# Udisks

An unofficial Rust client crate for [`udisks2`](https://github.com/storaged-project/udisks).
Udisks allows accessing and manipulating disks, storages devices and similar technologies. 

> [!WARNING]
> This project is very early in it's development cycle and far from being finished. Issues and API breaks should be expected.

## Example

```rust
// we use tokio in this example, but you can use any runtime
#[tokio::main]
async fn main() -> zbus::Result<()> {
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
