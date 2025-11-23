use crate::utils::open_device_or_fake;
use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use sed_manager::applications::RangeEditSession;
use sed_manager::rpc::TokioRuntime;
use sed_manager::spec::column_types::LockingRangeRef;
// use sed_manager::spec::objects::LockingRange;
use sed_manager::tper::TPer;
use std::sync::Arc;

use crate::utils::parse_uid;

#[derive(Args)]
pub struct RangeArgs {
    #[command(subcommand)]
    command: RangeCommand,

    /// The device to query (e.g., /dev/nvme0)
    #[arg(short, long)]
    device: String,

    /// Admin1 password (SID)
    #[arg(short, long, default_value = "")]
    password: String,
}

#[derive(Subcommand)]
pub enum RangeCommand {
    /// List all locking ranges
    List,
    /// Get details of a locking range
    Get {
        /// UID of the locking range (hex)
        uid: String,
    },
    /// Set properties of a locking range
    Set {
        /// UID of the locking range (hex)
        uid: String,
        #[arg(long)]
        range_start: Option<u64>,
        #[arg(long)]
        range_length: Option<u64>,
        #[arg(long)]
        read_lock_enabled: Option<bool>,
        #[arg(long)]
        write_lock_enabled: Option<bool>,
        #[arg(long)]
        read_locked: Option<bool>,
        #[arg(long)]
        write_locked: Option<bool>,
    },
    /// Cryptographically erase a locking range
    Erase {
        /// UID of the locking range (hex)
        uid: String,
    },
}

pub async fn run(args: RangeArgs) -> Result<()> {
    let device = open_device_or_fake(&args.device).context("Failed to open device")?;
    let device: Arc<dyn sed_manager::device::Device> = device.into();
    let runtime = Arc::new(TokioRuntime::new());
    let tper = TPer::new_on_default_com_id(device, runtime).context("Failed to create TPer")?;

    let session = RangeEditSession::start(&tper, args.password.as_bytes())
        .await
        .context("Failed to start RangeEditSession")?;

    match args.command {
        RangeCommand::List => {
            let ranges = session.list_ranges().await?;
            for range in ranges {
                println!("{}", range.as_uid());
            }
        }
        RangeCommand::Get { uid } => {
            let uid = parse_uid(&uid)?;
            let range_ref = LockingRangeRef::try_from(uid).map_err(|_| anyhow::anyhow!("Invalid LockingRange UID"))?;
            let range = session.get_range(range_ref).await?;
            println!("{:#?}", range);
        }
        RangeCommand::Set {
            uid,
            range_start,
            range_length,
            read_lock_enabled,
            write_lock_enabled,
            read_locked,
            write_locked,
        } => {
            let uid = parse_uid(&uid)?;
            let range_ref = LockingRangeRef::try_from(uid).map_err(|_| anyhow::anyhow!("Invalid LockingRange UID"))?;
            let mut range = session.get_range(range_ref).await?;

            if let Some(v) = range_start {
                range.range_start = v;
            }
            if let Some(v) = range_length {
                range.range_length = v;
            }
            if let Some(v) = read_lock_enabled {
                range.read_lock_enabled = v;
            }
            if let Some(v) = write_lock_enabled {
                range.write_lock_enabled = v;
            }
            if let Some(v) = read_locked {
                range.read_locked = v;
            }
            if let Some(v) = write_locked {
                range.write_locked = v;
            }

            session.set_range(&range).await?;
            println!("Range updated successfully.");
        }
        RangeCommand::Erase { uid } => {
            let uid = parse_uid(&uid)?;
            let range_ref = LockingRangeRef::try_from(uid).map_err(|_| anyhow::anyhow!("Invalid LockingRange UID"))?;
            session.erase_range(range_ref).await?;
            println!("Range erased successfully.");
        }
    }

    session.end().await?;
    Ok(())
}
