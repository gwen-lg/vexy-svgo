// this_file: crates/cli/src/commands/plugin/enable.rs

use anyhow::Result;
use crate::commands::plugin::PluginMarketplace;

pub async fn enable_plugin_command(marketplace: &PluginMarketplace, name: String) -> Result<()> {
    marketplace.set_plugin_enabled(&name, true)?;
    Ok(())
}
