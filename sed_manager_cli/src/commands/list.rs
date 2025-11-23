use anyhow::Result;
use sed_manager::device::list_physical_drives;

pub async fn run() -> Result<()> {
    let devices = list_physical_drives()?;

    for device in devices {
        println!("{:<20}", device);
    }Ok(())
}
