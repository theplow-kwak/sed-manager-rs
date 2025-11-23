use anyhow::{Context, Result};
use sed_manager::applications::{activate_locking, verify_locking_activation};
use sed_manager::device::open_device;
use sed_manager::rpc::TokioRuntime;
use sed_manager::tper::TPer;
use std::sync::Arc;

pub async fn run(device_path: String, sid_password: Option<String>, new_password: Option<String>) -> Result<()> {
    let device = open_device(&device_path).context("Failed to open device")?;
    let device: Arc<dyn sed_manager::device::Device> = device.into();
    let runtime = Arc::new(TokioRuntime::new());
    let tper = TPer::new_on_default_com_id(device, runtime).context("Failed to create TPer")?;

    let sid_password_bytes = sid_password.as_deref().map(|s| s.as_bytes()).unwrap_or(b"");
    let new_password_bytes = new_password.as_deref().map(|s| s.as_bytes());

    println!("Activating locking on {}...", device_path);
    activate_locking(&tper, sid_password_bytes, new_password_bytes).await?;

    println!("Verifying activation...");
    let verified = verify_locking_activation(&tper, new_password_bytes).await?;

    if verified {
        println!("Locking activated successfully.");
    } else {
        println!("Verification failed.");
    }

    Ok(())
}
