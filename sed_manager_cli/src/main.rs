use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available SED devices
    List,
    /// Activate locking on a device
    ActivateLocking {
        /// Device path (e.g., /dev/nvme0)
        device: String,
        /// SID password (default: MSID)
        #[arg(long)]
        sid_password: Option<String>,
        /// New Admin1 password
        #[arg(long)]
        new_password: Option<String>,
    },
    /// Change password for an authority
    ChangePassword {
        /// Device path
        device: String,
        /// Authority to change password for (e.g., Admin1)
        authority: String,
        /// Current password
        #[arg(long)]
        old_password: Option<String>,
        /// New password
        #[arg(long)]
        new_password: Option<String>,
    },
    /// Revert device to original factory state
    Revert {
        /// Device path
        device: String,
        /// PSID password (physical presence required)
        #[arg(long)]
        psid_password: String,
    },
    /// Take ownership of the device
    TakeOwnership {
        /// Device path
        device: String,
        /// SID password (default: MSID)
        #[arg(long)]
        sid_password: Option<String>,
        /// New SID password
        #[arg(long)]
        new_sid_password: Option<String>,
    },
    // TODO: Add other commands
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from_default_env()).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            commands::list::run().await?;
        }
        Commands::ActivateLocking { device, sid_password, new_password } => {
            commands::activate_locking::run(device, sid_password, new_password).await?;
        }
        Commands::ChangePassword { device, authority, old_password, new_password } => {
            commands::change_password::run(device, authority, old_password, new_password).await?;
        }
        Commands::Revert { device, psid_password } => {
            commands::revert::run(device, psid_password).await?;
        }
        Commands::TakeOwnership { device, sid_password, new_sid_password } => {
            commands::take_ownership::run(device, sid_password, new_sid_password).await?;
        }
    }

    Ok(())
}
