use crate::utils::open_device_or_fake;
use anyhow::{Context, Result};
use sed_manager::applications::get_admin_sp;
use sed_manager::applications::revert;
use sed_manager::rpc::TokioRuntime;
use sed_manager::spec::psid::admin::authority::PSID;
use sed_manager::tper::TPer;
use std::sync::Arc;

pub async fn run(device_path: String, psid_password: String) -> Result<()> {
    let device = open_device_or_fake(&device_path).context("Failed to open device")?;
    let device: Arc<dyn sed_manager::device::Device> = device.into();
    let runtime = Arc::new(TokioRuntime::new());
    let tper = TPer::new_on_default_com_id(device, runtime).context("Failed to create TPer")?;

    let discovery = tper.discover().await?;
    let ssc = discovery.get_primary_ssc().context("No available SSC")?;
    let admin_sp = get_admin_sp(ssc.feature_code())?;

    println!("Reverting device {} using PSID...", device_path);
    revert(&tper, PSID, psid_password.as_bytes(), admin_sp).await?;

    println!("Device reverted successfully.");

    Ok(())
}
