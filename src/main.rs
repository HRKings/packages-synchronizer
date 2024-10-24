pub mod cli;
pub mod config;
pub mod runner;

use anyhow::Result;
use clap::Parser;
use cli::{RawCommand, RawOpt};
use config::load_config;
use runner::{
    get_current_packages, get_ignored_packages, get_tracked_packages, get_untracked_packages,
};

fn main() -> Result<()> {
    let options = RawOpt::parse();

    let config = load_config("./assets/groups/")?;

    match options.command {
        RawCommand::ListManagers { list_groups } => {
            for (manager_name, manager) in config.managers {
                println!("- {}:", manager_name);

                if list_groups {
                    println!("  {:?}", manager.packages.keys());
                }
            }
        }
        RawCommand::Untracked { manager } => print!(
            "{:?}",
            get_untracked_packages(&config.managers[manager.as_str()])
        ),
        RawCommand::Tracked { manager } => print!(
            "{:?}",
            get_tracked_packages(&config.managers[manager.as_str()])
        ),
        RawCommand::Current { manager } => print!(
            "{:?}",
            get_current_packages(&config.managers[manager.as_str()])
        ),
        RawCommand::Ignored { manager } => print!(
            "{:?}",
            get_ignored_packages(&config.managers[manager.as_str()])
        ),
        RawCommand::Sync {
            manager,
            uninstall_untracked,
        } => todo!(),
    }

    Ok(())
}
