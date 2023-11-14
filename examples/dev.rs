use zbus::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = udisks_rs::Client::new().await?;
    let version = client.manager().version().await?;
    println!("Version: {}", version);
    let object = client
        .get_object("/org/freedesktop/UDisks2/block_devices/sda")
        .await
        .unwrap();
    Ok(())
}
