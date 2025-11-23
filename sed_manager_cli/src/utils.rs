use anyhow::{anyhow, Context, Result};
use sed_manager::messaging::uid::UID;

pub fn parse_uid(s: &str) -> Result<UID> {
    let s = s.trim().replace("_", "");
    let s = s.trim_start_matches("0x");

    if s.len() != 16 {
        return Err(anyhow!("Invalid UID length: expected 16 hex digits, got {}", s.len()));
    }

    let value = u64::from_str_radix(&s, 16).context("Failed to parse UID hex string")?;
    Ok(UID::new(value))
}
