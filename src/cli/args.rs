use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
#[command(disable_help_flag = true)]
pub(super) struct ContainerArgs {
    /// Container instance identifier, not a process PID.
    container_id: String,
    /// Print full help for this command.
    #[arg(short, long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Args)]
pub(super) struct CreateArgs {
    #[command(flatten)]
    cont_args: ContainerArgs,
    /// OCI bundle directory containing config.json.
    bundle_path: PathBuf,
}

#[derive(Args)]
pub(super) struct RunArgs {
    #[command(flatten)]
    cont_args: ContainerArgs,
    /// OCI bundle directory; required when the container ID is new.
    bundle_path: Option<PathBuf>,
}
