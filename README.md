# UDisks

An unofficial Rust client crate for [`udisks2`](https://github.com/storaged-project/udisks).
UDisks allows accessing and manipulating disks, storage devices and similar technologies.

`udisks` need to be installed and accessible.

## Version

Based on `udsisks` version [`2.10.2`](https://github.com/storaged-project/udisks/releases/tag/udisks-2.10.2).

## Example

```rust
// we use tokio in this example, but you can use any runtime
#[tokio::main]
async fn main() -> udisks2::Result<()> {
    let client = udisks2::Client::new().await?;
    let Ok(object) = client.object("/org/freedesktop/UDisks2/block_devices/sda") else {
        eprintln!("No sda device found");
        return Ok(());
    };
    let block = object.block().await?;
    let drive = client.drive_for_block(&block).await?;
    println!(
        "Size: {}",
        client.size_for_display(drive.size().await?, true, true)
    );
    Ok(())
}
```

### Internationalization

Some functions in the crate provide the option to return translated text, which may be directly used in user interface. This crate uses the same localization as `UDisks2` (via [`gettext`](https://github.com/gettext-rs/gettext-rs)). If the locale is left unset, English will be used.

To setup localization, to e.g. German:
```rust
gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "de_DE.UTF-8");
gettextrs::textdomain("udisks2").expect("Failed to set textdomain");
```
