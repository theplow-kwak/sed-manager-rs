use crate::utils::open_device_or_fake;
use anyhow::Result;
use clap::Args;
use sed_manager::rpc::discover;

#[derive(Args)]
pub struct DiscoveryArgs {
    /// The device to query (e.g., /dev/nvme0)
    device: String,
}

pub async fn run(args: DiscoveryArgs) -> Result<()> {
    let device = open_device_or_fake(&args.device)?;
    let discovery = discover(&*device)?;
    println!("{:#?}", discovery);
    Ok(())
}
