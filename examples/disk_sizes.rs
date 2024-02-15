#[tokio::main]
async fn main() -> zbus::Result<()> {
    let client = udisks2::Client::new().await?;

    for object in client
        .object_manager()
        .get_managed_objects()
        .await?
        .into_iter()
        .filter_map(|(object_path, _)| client.object(object_path).ok())
    {
        //only use objects that have a drive
        let Ok(drive) = object.drive().await else {
            continue;
        };

        // print model and size
        println!(
            "{}: {}",
            drive.model().await?,
            client.size_for_display(drive.size().await?, false, false)
        );
    }
    Ok(())
}
