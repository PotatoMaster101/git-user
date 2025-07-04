use clap::{Parser, Subcommand};

/// Represents CLI arguments.
#[derive(Clone, Debug, Parser)]
#[command(name = "git-user", about = "Manage Git users.")]
pub struct Args {
    /// CLI subcommand.
    #[command(subcommand)]
    pub command: Command,

    /// Custom config file path.
    #[arg(short, long, global = true, help = "Custom config file path", default_value = "~/.gitusers")]
    pub config: String,
}

/// Represents a CLI subcommand.
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Subcommand for creating a new user.
    #[command(about = "Create a new user", alias = "a")]
    Add {
        /// Git username.
        name: String,

        /// Git user email address.
        email: String,

        /// Custom profile name for this user.
        #[arg(short, long)]
        profile: Option<String>,

        /// Git user signing key.
        #[arg(short = 'k', long)]
        signing_key: Option<String>,

        /// SSH command to run when authenticating.
        #[arg(short = 's', long)]
        ssh_command: Option<String>,
    },

    /// Subcommand for deleting a user.
    #[command(about = "Deletes a user", alias = "d")]
    Delete {
        /// User profile to delete.
        profile: String,
    },

    /// Subcommand for exporting the config.
    #[command(about = "Export config", alias = "e")]
    Export,

    /// Subcommand for listing users.
    #[command(about = "List users", alias = "l")]
    List,

    /// Subcommand for using a specific user.
    #[command(about = "Use user", alias = "u")]
    Use {
        /// User profile to use.
        profile: String,

        /// Git repo path.
        #[arg(short, long, help = "Git repository path", default_value = ".")]
        repo: String,
    }
}
