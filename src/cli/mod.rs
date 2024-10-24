pub mod subcommands;

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, PartialEq, Eq, Parser)]
#[command(author, version, about, long_about = None)]
pub struct RawOpt {
    /// Suppress any informational output.
    #[clap(long, short)]
    pub quiet: bool,

    /// Suppress any interactive prompts and assume "yes" as the answer.
    #[clap(long)]
    pub non_interactive: bool,

    /// Use verbose output.
    #[clap(long, short)]
    pub verbose: bool,

    /// The data directory
    #[clap(long, value_name = "PATH", env = "DATA_DIR")]
    pub data_dir: Option<PathBuf>,

    /// The config file.
    #[clap(long, value_name = "PATH", env = "CONFIG_FILE")]
    pub config_file: Option<PathBuf>,

    /// The subcommand to run.
    #[clap(subcommand)]
    pub command: RawCommand,
}

#[derive(Debug, PartialEq, Eq, Parser)]
pub enum RawCommand {
    /// Show which managers are loaded from the config
    ListManagers {
        // If the sync should delete untracked packages
        #[clap(long, short = 'g')]
        list_groups: bool,
    },
    /// Show what packages are currently untracked
    Untracked {
        /// The manager to filter
        #[clap(value_name = "MANAGER")]
        manager: String,
    },
    /// Show what packages are currently tracked
    Tracked {
        /// The manager to filter
        #[clap(value_name = "MANAGER")]
        manager: String,
    },
    /// Show what packages are currently installed
    Current {
        /// The manager to filter
        #[clap(value_name = "MANAGER")]
        manager: String,
    },
    /// Show what packages are currently ignored
    Ignored {
        /// The manager to filter
        #[clap(long, value_name = "MANAGER")]
        manager: String,
    },
    /// Sync packages. Optionally specifying what manager should be searched. If prompted, will uninstall untracked packages
    Sync {
        /// The manager to filter
        #[clap(long, value_name = "MANAGER")]
        manager: Option<String>,
        /// If the sync should delete untracked packages
        #[clap(long, short)]
        uninstall_untracked: bool,
    },
}
