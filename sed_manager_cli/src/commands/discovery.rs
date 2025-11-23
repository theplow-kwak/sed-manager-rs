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
    print_discovery(&discovery);
    Ok(())
}

fn print_discovery(discovery: &sed_manager::messaging::discovery::Discovery) {
    use sed_manager::messaging::discovery::{Feature, FeatureDescriptor};

    println!("Discovery Data:");
    println!();

    for feature in discovery.iter() {
        println!("Feature: {} (Version {})", feature.feature_code(), feature.version());
        match feature {
            FeatureDescriptor::TPer(desc) => {
                println!("  ComID Management Supported: {}", desc.com_id_mgmt_supported);
                println!("  Streaming Supported: {}", desc.streaming_supported);
                println!("  Buffer Management Supported: {}", desc.buffer_mgmt_supported);
                println!("  ACK/NAK Supported: {}", desc.ack_nak_supported);
                println!("  Async Supported: {}", desc.async_supported);
                println!("  Sync Supported: {}", desc.sync_supported);
            }
            FeatureDescriptor::Locking(desc) => {
                println!("  Locking Supported: {}", desc.locking_supported);
                println!("  Locking Enabled: {}", desc.locking_enabled);
                println!("  Locked: {}", desc.locked);
                println!("  Media Encryption: {}", desc.media_encryption);
                println!("  MBR Enabled: {}", desc.mbr_enabled);
                println!("  MBR Done: {}", desc.mbr_done);
                println!("  MBR Shadowing Not Supported: {}", desc.mbr_shadowing_not_supported);
                println!("  HW Reset Supported: {}", desc.hw_reset_supported);
            }
            FeatureDescriptor::Geometry(desc) => {
                println!("  Align: {}", desc.align);
                println!("  Logical Block Size: {}", desc.logical_block_size);
                println!("  Alignment Granularity: {}", desc.alignment_granularity);
                println!("  Lowest Aligned LBA: {}", desc.lowest_aligned_lba);
            }
            FeatureDescriptor::DataRemoval(desc) => {
                println!("  Processing: {}", desc.processing);
                println!("  Interrupted: {}", desc.interrupted);
                println!("  Supported Mechanisms:");
                println!("    Overwrite: {}", desc.supported_mechanism.overwrite);
                println!("    Block Erase: {}", desc.supported_mechanism.block_erase);
                println!("    Crypto Erase: {}", desc.supported_mechanism.crypto_erase);
                println!("    Vendor Erase: {}", desc.supported_mechanism.vendor_erase);
            }
            FeatureDescriptor::BlockSIDAuth(desc) => {
                println!("  Locking SP Frozen: {}", desc.locking_sp_frozen);
                println!("  Locking SP Freeze Supported: {}", desc.locking_sp_freeze_supported);
                println!("  SID Authentication Blocked: {}", desc.sid_authentication_blocked);
                println!("  SID/MSID PIN Differ: {}", desc.sid_msid_pin_differ);
                println!("  HW Reset Unblocks: {}", desc.hw_reset_unblocks);
            }
            FeatureDescriptor::OpalV2(desc) => {
                println!("  Base ComID: {}", desc.base_com_id);
                println!("  Num ComIDs: {}", desc.num_com_ids);
                println!("  No Range Crossing: {}", desc.no_range_crossing);
                println!("  Num Locking Admins: {}", desc.num_locking_admins_supported);
                println!("  Num Locking Users: {}", desc.num_locking_users_supported);
                println!("  Initial Owner Password: {:?}", desc.initial_owner_pw);
                println!("  Reverted Owner Password: {:?}", desc.reverted_owner_pw);
            }
            FeatureDescriptor::PyriteV1(desc) => {
                println!("  Base ComID: {}", desc.base_com_id);
                println!("  Num ComIDs: {}", desc.num_com_ids);
                println!("  Initial Owner Password: {:?}", desc.initial_owner_pw);
                println!("  Reverted Owner Password: {:?}", desc.reverted_owner_pw);
            }
            FeatureDescriptor::PyriteV2(desc) => {
                println!("  Base ComID: {}", desc.base_com_id);
                println!("  Num ComIDs: {}", desc.num_com_ids);
                println!("  Initial Owner Password: {:?}", desc.initial_owner_pw);
                println!("  Reverted Owner Password: {:?}", desc.reverted_owner_pw);
            }
            FeatureDescriptor::Enterprise(desc) => {
                println!("  Base ComID: {}", desc.base_com_id);
                println!("  Num ComIDs: {}", desc.num_com_ids);
                println!("  No Range Crossing: {}", desc.no_range_crossing);
            }
            FeatureDescriptor::OpalV1(desc) => {
                println!("  Base ComID: {}", desc.base_com_id);
                println!("  Num ComIDs: {}", desc.num_com_ids);
                println!("  No Range Crossing: {}", desc.no_range_crossing);
            }
            FeatureDescriptor::Opalite(desc) => {
                println!("  Base ComID: {}", desc.base_com_id);
                println!("  Num ComIDs: {}", desc.num_com_ids);
                println!("  Initial Owner Password: {:?}", desc.initial_owner_pw);
                println!("  Reverted Owner Password: {:?}", desc.reverted_owner_pw);
            }
            FeatureDescriptor::Ruby(desc) => {
                println!("  Base ComID: {}", desc.base_com_id);
                println!("  Num ComIDs: {}", desc.num_com_ids);
                println!("  No Range Crossing: {}", desc.no_range_crossing);
                println!("  Num Locking Admins: {}", desc.num_locking_admins_supported);
                println!("  Num Locking Users: {}", desc.num_locking_users_supported);
                println!("  Initial Owner Password: {:?}", desc.initial_owner_pw);
                println!("  Reverted Owner Password: {:?}", desc.reverted_owner_pw);
            }
            FeatureDescriptor::KeyPerIO(desc) => {
                println!("  Base ComID P1: {}", desc.base_com_id_p1);
                println!("  Num ComIDs P1: {}", desc.num_com_ids_p1);
                println!("  Base ComID P3: {}", desc.base_com_id_p3);
                println!("  Num ComIDs P3: {}", desc.num_com_ids_p3);
                // Add more fields if necessary
            }
            FeatureDescriptor::AdditionalDataStoreTables(desc) => {
                println!("  Max Num Tables: {}", desc.max_num_tables);
                println!("  Max Total Size: {}", desc.max_total_size_of_tables);
                println!("  Table Size Alignment: {}", desc.table_size_alignment);
            }
            FeatureDescriptor::Unrecognized(desc) => {
                println!("  Length: {}", desc.length);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sed_manager::messaging::discovery::{Discovery, FeatureDescriptor, LockingDescriptor, TPerDescriptor};

    #[test]
    fn test_print_discovery() {
        let tper = FeatureDescriptor::TPer(TPerDescriptor {
            com_id_mgmt_supported: true,
            streaming_supported: true,
            buffer_mgmt_supported: false,
            ack_nak_supported: false,
            async_supported: true,
            sync_supported: true,
        });

        let locking = FeatureDescriptor::Locking(LockingDescriptor {
            hw_reset_supported: true,
            mbr_shadowing_not_supported: false,
            mbr_done: true,
            mbr_enabled: false,
            media_encryption: true,
            locked: false,
            locking_enabled: true,
            locking_supported: true,
        });

        let discovery = Discovery::new(vec![tper, locking]);

        // Just verify it doesn't panic
        print_discovery(&discovery);
    }
}
