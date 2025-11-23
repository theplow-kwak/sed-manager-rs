use anyhow::{Context, Result};
use sed_manager::applications::{take_ownership, verify_ownership};
use sed_manager::device::open_device;
use sed_manager::rpc::TokioRuntime;
use sed_manager::tper::TPer;
use std::sync::Arc;

pub async fn run(device_path: String, _sid_password: Option<String>, new_sid_password: Option<String>) -> Result<()> {
    let device = open_device(&device_path).context("Failed to open device")?;
    let device: Arc<dyn sed_manager::device::Device> = device.into();
    let runtime = Arc::new(TokioRuntime::new());
    let tper = TPer::new_on_default_com_id(device, runtime).context("Failed to create TPer")?;

    let new_sid_password_bytes = new_sid_password.as_deref().map(|s| s.as_bytes());

    let password = new_sid_password_bytes.unwrap_or(b"");
    println!("Taking ownership of {}...", device_path);
    take_ownership(&tper, password).await?;
    
    println!("Verifying ownership...");
    let verified = verify_ownership(&tper, password).await?;

    if verified {
        println!("Ownership taken successfully.");
    } else {
        println!("Verification failed.");
    }

    Ok(())
}
