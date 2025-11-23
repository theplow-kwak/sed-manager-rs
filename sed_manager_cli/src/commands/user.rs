use crate::utils::open_device_or_fake;
use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use sed_manager::applications::UserEditSession;
use sed_manager::rpc::TokioRuntime;
use sed_manager::spec::column_types::AuthorityRef;
use sed_manager::tper::TPer;
use std::sync::Arc;

use crate::utils::parse_uid;

#[derive(Args)]
pub struct UserArgs {
    #[command(subcommand)]
    command: UserCommand,

    /// The device to query (e.g., /dev/nvme0)
    #[arg(short, long)]
    device: String,

    /// Admin1 password (SID)
    #[arg(short, long, default_value = "")]
    password: String,
}

#[derive(Subcommand)]
pub enum UserCommand {
    /// List all users
    List,
    /// Get details of a user
    Get {
        /// UID of the user (hex)
        uid: String,
    },
    /// Enable a user
    Enable {
        /// UID of the user (hex)
        uid: String,
    },
    /// Disable a user
    Disable {
        /// UID of the user (hex)
        uid: String,
    },
    /// Set the common name of a user
    SetName {
        /// UID of the user (hex)
        uid: String,
        /// New name
        name: String,
    },
    /// Set the password of a user
    SetPassword {
        /// UID of the user (hex)
        uid: String,
        /// New password
        #[arg(long)]
        new_password: String,
    },
}

pub async fn run(args: UserArgs) -> Result<()> {
    let device = open_device_or_fake(&args.device).context("Failed to open device")?;
    let device: Arc<dyn sed_manager::device::Device> = device.into();
    let runtime = Arc::new(TokioRuntime::new());
    let tper = TPer::new_on_default_com_id(device, runtime).context("Failed to create TPer")?;

    let session = UserEditSession::start(&tper, args.password.as_bytes())
        .await
        .context("Failed to start UserEditSession")?;

    match args.command {
        UserCommand::List => {
            let users = session.list_users().await?;
            for user in users {
                println!("{}", user.as_uid());
            }
        }
        UserCommand::Get { uid } => {
            let uid = parse_uid(&uid)?;
            let user_ref = AuthorityRef::try_from(uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            let user = session.get_user(user_ref).await?;
            println!("UID: {}", user.uid.as_uid());
            println!("Common Name: {:?}", user.common_name);
            println!("Enabled: {}", user.enabled);
        }
        UserCommand::Enable { uid } => {
            let uid = parse_uid(&uid)?;
            let user_ref = AuthorityRef::try_from(uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            session.set_enabled(user_ref, true).await?;
            println!("User enabled successfully.");
        }
        UserCommand::Disable { uid } => {
            let uid = parse_uid(&uid)?;
            let user_ref = AuthorityRef::try_from(uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            session.set_enabled(user_ref, false).await?;
            println!("User disabled successfully.");
        }
        UserCommand::SetName { uid, name } => {
            let uid = parse_uid(&uid)?;
            let user_ref = AuthorityRef::try_from(uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            session.set_name(user_ref, &name).await?;
            println!("User name updated successfully.");
        }
        UserCommand::SetPassword { uid, new_password } => {
            let uid = parse_uid(&uid)?;
            let user_ref = AuthorityRef::try_from(uid).map_err(|_| anyhow::anyhow!("Invalid User UID"))?;
            session.set_password(user_ref, new_password.as_bytes()).await?;
            println!("User password updated successfully.");
        }
    }

    session.end().await?;
    Ok(())
}
