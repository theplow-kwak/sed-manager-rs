use anyhow::{Context, Result, anyhow};
use sed_manager::applications::{change_password, list_password_authorities};
use sed_manager::applications::get_general_lookup;
use sed_manager::device::open_device;
use sed_manager::tper::TPer;
use sed_manager::rpc::TokioRuntime;
use std::sync::Arc;

pub async fn run(
    device_path: String, 
    authority_name: String, 
    old_password: Option<String>, 
    new_password: Option<String>
) -> Result<()> {
    let device = open_device(&device_path).context("Failed to open device")?;
    let device: Arc<dyn sed_manager::device::Device> = device.into();
    let runtime = Arc::new(TokioRuntime::new());
    let tper = TPer::new_on_default_com_id(device, runtime).context("Failed to create TPer")?;

    let discovery = tper.discover().await?;
    let ssc = discovery.get_primary_ssc().context("No available SSC")?;
    let lookup = get_general_lookup(ssc.feature_code());

    // Find the authority and its SP
    let authorities = list_password_authorities(&tper).await?;
    let mut target = None;

    for (sp, auth) in authorities {
        let name = lookup.by_uid(auth.as_uid(), None);
        if let Some(name) = name {
            if name.eq_ignore_ascii_case(&authority_name) {
                target = Some((sp, auth));
                break;
            }
        }
    }

    let (sp, authority) = target.ok_or_else(|| anyhow!("Authority '{}' not found or not supported", authority_name))?;

    let old_password_bytes = old_password.as_deref().map(|s| s.as_bytes()).unwrap_or(b"");
    let new_password_bytes = new_password.as_deref().map(|s| s.as_bytes()).unwrap_or(b"");

    println!("Changing password for {}...", authority_name);
    change_password(&tper, sp, authority, old_password_bytes, new_password_bytes).await?;
    
    println!("Password changed successfully.");

    Ok(())
}
