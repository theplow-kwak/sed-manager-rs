use anyhow::{anyhow, Context, Result};
use sed_manager::device::{open_device, Device};
use sed_manager::fake_device::FakeDevice;
use sed_manager::messaging::uid::UID;
use std::sync::Arc;

pub fn parse_uid(s: &str) -> Result<UID> {
    let s = s.trim().replace("_", "");
    let s = s.trim_start_matches("0x");

    if s.len() != 16 {
        return Err(anyhow!("Invalid UID length: expected 16 hex digits, got {}", s.len()));
    }

    let value = u64::from_str_radix(&s, 16).context("Failed to parse UID hex string")?;
    Ok(UID::new(value))
}

pub fn open_device_or_fake(device_path: &str) -> Result<Arc<dyn Device>> {
    if device_path.eq_ignore_ascii_case("fake") || device_path.eq_ignore_ascii_case("fake_device") {
        Ok(Arc::new(FakeDevice::new()))
    } else {
        let device = open_device(device_path)?;
        Ok(device.into())
    }
}
