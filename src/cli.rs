mod args;

use args::{ContainerArgs, CreateArgs, RunArgs};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "capsa",
    version,
    about,
    long_about = None,
    arg_required_else_help = true,
    disable_help_flag = true,
    after_help = "Example:\n  capsa create demo ./my-bundle\n\nFor full help, use 'capsa --help' or 'capsa help <command>'.",
    after_long_help = "Examples:\n  capsa create demo ./my-bundle\n  capsa start demo\n  capsa run fresh ./my-bundle\n\nUse 'capsa help <command>' for command details and examples.\nDocumentation and feedback: https://github.com/ammartahamohamedy/capsa\n\nThis build parses requests; container execution is not implemented.",
)]
pub(crate) struct Cli {
    /// Print full help.
    #[arg(short, long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inspect a container's identity, lifecycle state, and pause condition.
    #[command(
        long_about = "Inspect an existing container instance by its container ID. State identifies the instance and reports its lifecycle status and separate pause condition; it does not select a process by PID.",
        after_help = "Example:\n  capsa state demo\n\nThis build parses requests; container execution is not implemented."
    )]
    State(ContainerArgs),
    /// Prepare a container from an OCI bundle without executing its workload.
    #[command(
        long_about = "Prepare a new container instance from an OCI bundle containing config.json and referencing a prepared root filesystem. Creation freezes the instance's configuration without executing the workload. The container ID must not already exist.",
        after_help = "Examples:\n  capsa create demo ./my-bundle\n  capsa start demo\n\nThis build parses requests; container execution is not implemented."
    )]
    Create(CreateArgs),
    /// Execute the workload of a prepared container.
    #[command(
        long_about = "Execute the workload of a container in the created state using its frozen configuration. Start does not accept replacement configuration, resume a paused workload, or relaunch a stopped instance.",
        after_help = "Example:\n  capsa start demo\n\nThis build parses requests; container execution is not implemented."
    )]
    Start(ContainerArgs),
    /// Start a prepared container, or create and start a new one.
    #[command(
        long_about = "Look up the container ID first. For an existing created instance, start its workload using its frozen configuration. For a new ID, require a bundle and compose create followed by start. Creating, running, paused, and stopped instances are not relaunched. A supplied bundle does not replace an existing instance's configuration.",
        after_help = "Examples:\n  capsa run prepared\n  capsa run fresh ./my-bundle\n\nThis build parses requests; container execution is not implemented."
    )]
    Run(RunArgs),
    /// Suspend a running workload while retaining its live execution state.
    #[command(
        long_about = "Suspend a running workload's processes and threads while retaining their memory and execution environment. Pause does not terminate the workload or save a checkpoint. Resume continues the same live workload.",
        after_help = "Example:\n  capsa pause demo\n\nThis build parses requests; container execution is not implemented."
    )]
    Pause(ContainerArgs),
    /// Continue the same paused workload.
    #[command(
        long_about = "Continue a paused workload without creating a new process or re-executing its command. Resume cannot restore a workload that has terminated.",
        after_help = "Example:\n  capsa resume demo\n\nThis build parses requests; container execution is not implemented."
    )]
    Resume(ContainerArgs),
    /// Request shutdown and escalate if the workload does not exit in time.
    #[command(
        long_about = "Request confirmation before sending SIGTERM, wait for the grace period, then escalate to SIGKILL if needed and observe termination. Confirmation defaults to No. An explicit bypass skips only confirmation, not validation or the grace period. Termination leaves the instance available for inspection; the exited workload cannot be resumed.",
        after_help = "Example:\n  capsa terminate demo\n\nThis build parses requests; container execution is not implemented."
    )]
    Terminate(ContainerArgs),
    /// Delete an eligible stopped container and release its owned resources.
    #[command(
        long_about = "Check that the selected container is stopped, then delete that same instance and release its remaining owned resources. Remove does not implicitly terminate a live workload or delete external rootfs data and shared image resources.",
        after_help = "Example:\n  capsa remove demo\n\nThis build parses requests; container execution is not implemented."
    )]
    Remove(ContainerArgs),
}
