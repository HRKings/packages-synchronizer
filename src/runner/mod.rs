use anyhow::Result;
use cmd_lib::run_fun;
use walkdir::WalkDir;

use crate::config::{from_path, RawConfig, RawManager};

pub fn load_config(path: &str) -> Result<RawConfig> {
    let mut result: RawConfig = RawConfig::default();

    for entry_result in WalkDir::new(path).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            return true;
        }

        e.path()
            .to_str()
            .map(|s| s.ends_with("toml"))
            .unwrap_or(true)
    }) {
        match entry_result {
            Ok(entry) if entry.file_type().is_file() => {
                let mut tmp = from_path(entry.path())?;
                result.managers.append(&mut tmp.managers);
            }
            _ => (),
        };
    }

    Ok(result)
}

pub fn get_current_packages(manager: &RawManager) -> Result<Vec<String>> {
    let command = manager.list_current_command.as_str();
    let result = run_fun!(bash -c $command)?;

    Ok(result.split('\n').map(|v| v.to_string()).collect())
}

pub fn get_ignored_packages(manager: &RawManager) -> Result<Option<Vec<String>>> {
    if manager.list_ignored_command.is_none() {
        return Ok(None);
    }

    let command = manager.list_ignored_command.clone().unwrap();
    let result = run_fun!(bash -c $command)?;

    Ok(Some(result.split('\n').map(|v| v.to_string()).collect()))
}

pub fn get_tracked_packages(manager: &RawManager) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();

    for package_list in manager.packages.values() {
        result.append(package_list.clone().as_mut());
    }

    Ok(result)
}

pub fn get_untracked_packages(manager: &RawManager) -> Result<Vec<String>> {
    let tracked = get_tracked_packages(manager)?;
    let ignored = get_ignored_packages(manager)?;
    let current = get_current_packages(manager)?;

    let mut result: Vec<String> = Vec::new();

    for pkg in current {
        if !tracked.contains(&pkg) && ignored.as_ref().is_some_and(|i| !i.contains(&pkg)) {
            result.push(pkg);
        }
    }

    Ok(result)
}
