use anyhow::Result;
use cmd_lib::run_fun;

use crate::config::RawManager;

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
        if !tracked.contains(&pkg) || ignored.as_ref().is_some_and(|i| !i.contains(&pkg)) {
            result.push(pkg);
        }
    }

    Ok(result)
}
