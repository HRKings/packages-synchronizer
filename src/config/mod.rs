use anyhow::{Context as ResultExt, Result};
use indexmap::IndexMap;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fs;
use std::path::Path;

/// The contents of the configuration file.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RawConfig {
    /// A map of name to plugin.
    pub managers: IndexMap<String, RawManager>,
}

/// The actual manager configuration.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct RawManager {
    pub list_ignored_command: Option<String>,
    pub list_current_command: String,
    pub install_command: String,
    pub uninstall_command: Option<String>,
    #[serde(flatten, deserialize_with = "deserialize_rest_toml_value")]
    pub packages: IndexMap<String, Vec<String>>,
}

/// Deserialize the remaining keys into an [`Option<toml::Value>`]. Empty tables
/// are coerced to [`None`].
fn deserialize_rest_toml_value<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: toml::Value = de::Deserialize::deserialize(deserializer)?;

    let table = match value {
        toml::Value::Table(table) if !table.is_empty() => table,
        _ => panic!("Package group cannot be found."),
    };

    let mut result: IndexMap<String, Vec<String>> = IndexMap::new();

    for table in table {
        let inner_table = match table.1 {
            toml::Value::Table(inner_table) if !inner_table.is_empty() => inner_table,
            _ => panic!("Package group cannot be found."),
        };

        let Some(packages_value) = inner_table.get("packages") else {
            panic!("Package list cannot be found.");
        };

        let toml::Value::Array(packages) = packages_value else {
            panic!("Package list cannot be found.");
        };

        let mut inner_array: Vec<String> = Vec::new();

        for value in packages {
            let toml::Value::String(array_item) = value else {
                panic!("Package list contains a value that is not a string.");
            };

            inner_array.push(array_item.clone());
        }

        result.insert(table.0, inner_array);
    }

    Ok(result)
}

/// Load a [`RawConfig`] from the given path.
pub fn from_path<P>(path: P) -> Result<RawConfig>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let bytes =
        fs::read(path).with_context(|| format!("failed to read from `{}`", path.display()))?;
    let contents = String::from_utf8(bytes).context("config file contents are not valid UTF-8")?;

    let config: RawConfig =
        toml::from_str(&contents).context("failed to deserialize contents as TOML")?;

    Ok(config)
}
