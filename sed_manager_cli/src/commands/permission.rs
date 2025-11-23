use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use sed_manager::applications::PermissionEditSession;
use sed_manager::device::open_device;
use sed_manager::rpc::TokioRuntime;
use sed_manager::spec::column_types::{AuthorityRef, LockingRangeRef};
use sed_manager::tper::TPer;
use std::sync::Arc;

use crate::utils::parse_uid;

#[derive(Args)]
pub struct PermissionArgs {
    #[command(subcommand)]
    command: PermissionCommand,

    /// The device to query (e.g., /dev/nvme0)
    #[arg(short, long)]
    device: String,

    /// Admin1 password (SID)
    #[arg(short, long, default_value = "")]
    password: String,
}

#[derive(Subcommand)]
pub enum PermissionCommand {
    /// List users available for permission editing
    ListUsers,
    /// List ranges available for permission editing
    ListRanges,
    /// Get MBR permission for a user
    GetMbr {
        /// UID of the user (hex)
        user_uid: String,
    },
    /// Get read permission for a user on a range
    GetRead {
        /// UID of the user (hex)
        user_uid: String,
        /// UID of the range (hex)
        range_uid: String,
    },
    /// Get write permission for a user on a range
    GetWrite {
        /// UID of the user (hex)
        user_uid: String,
        /// UID of the range (hex)
        range_uid: String,
    },
    /// Set MBR permission for a user
    SetMbr {
        /// UID of the user (hex)
        user_uid: String,
        /// Allow (true) or deny (false)
        allow: bool,
    },
    /// Set read permission for a user on a range
    SetRead {
        /// UID of the user (hex)
        user_uid: String,
        /// UID of the range (hex)
        range_uid: String,
        /// Allow (true) or deny (false)
        allow: bool,
    },
    /// Set write permission for a user on a range
    SetWrite {
        /// UID of the user (hex)
        user_uid: String,
        /// UID of the range (hex)
        range_uid: String,
        /// Allow (true) or deny (false)
        allow: bool,
    },
}

pub async fn run(args: PermissionArgs) -> Result<()> {
    let device = open_device(&args.device).context("Failed to open device")?;
    let device: Arc<dyn sed_manager::device::Device> = device.into();
    let runtime = Arc::new(TokioRuntime::new());
    let tper = TPer::new_on_default_com_id(device, runtime).context("Failed to create TPer")?;

    let session = PermissionEditSession::start(&tper, args.password.as_bytes())
        .await
        .context("Failed to start PermissionEditSession")?;

    match args.command {
        PermissionCommand::ListUsers => {
            let users = session.list_users().await?;
            for user in users {
                println!("{}", user.as_uid());
            }
        }
        PermissionCommand::ListRanges => {
            let ranges = session.list_ranges().await?;
            for range in ranges {
                println!("{}", range.as_uid());
            }
        }
        PermissionCommand::GetMbr { user_uid } => {
            let user_uid = parse_uid(&user_uid)?;
            let user_ref = AuthorityRef::try_from(user_uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            let allowed = session.get_mbr_permission(user_ref).await?;
            println!("MBR Permission: {}", allowed);
        }
        PermissionCommand::GetRead { user_uid, range_uid } => {
            let user_uid = parse_uid(&user_uid)?;
            let user_ref = AuthorityRef::try_from(user_uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            let range_uid = parse_uid(&range_uid)?;
            let range_ref = LockingRangeRef::try_from(range_uid).map_err(|_| anyhow::anyhow!("Invalid Range UID"))?;
            let allowed = session.get_read_permission(user_ref, range_ref).await?;
            println!("Read Permission: {}", allowed);
        }
        PermissionCommand::GetWrite { user_uid, range_uid } => {
            let user_uid = parse_uid(&user_uid)?;
            let user_ref = AuthorityRef::try_from(user_uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            let range_uid = parse_uid(&range_uid)?;
            let range_ref = LockingRangeRef::try_from(range_uid).map_err(|_| anyhow::anyhow!("Invalid Range UID"))?;
            let allowed = session.get_write_permission(user_ref, range_ref).await?;
            println!("Write Permission: {}", allowed);
        }
        PermissionCommand::SetMbr { user_uid, allow } => {
            let user_uid = parse_uid(&user_uid)?;
            let user_ref = AuthorityRef::try_from(user_uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            session.set_mbr_permission(user_ref, allow).await?;
            println!("MBR Permission updated successfully.");
        }
        PermissionCommand::SetRead { user_uid, range_uid, allow } => {
            let user_uid = parse_uid(&user_uid)?;
            let user_ref = AuthorityRef::try_from(user_uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            let range_uid = parse_uid(&range_uid)?;
            let range_ref = LockingRangeRef::try_from(range_uid).map_err(|_| anyhow::anyhow!("Invalid Range UID"))?;
            session.set_read_permission(user_ref, range_ref, allow).await?;
            println!("Read Permission updated successfully.");
        }
        PermissionCommand::SetWrite { user_uid, range_uid, allow } => {
            let user_uid = parse_uid(&user_uid)?;
            let user_ref = AuthorityRef::try_from(user_uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            let range_uid = parse_uid(&range_uid)?;
            let range_ref = LockingRangeRef::try_from(range_uid).map_err(|_| anyhow::anyhow!("Invalid Range UID"))?;
            session.set_write_permission(user_ref, range_ref, allow).await?;
            println!("Write Permission updated successfully.");
        }
    }

    session.end().await?;
    Ok(())
}
