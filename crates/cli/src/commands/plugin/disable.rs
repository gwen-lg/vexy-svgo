// this_file: crates/cli/src/commands/plugin/disable.rs

use anyhow::Result;
use crate::commands::plugin::PluginMarketplace;

pub async fn disable_plugin_command(marketplace: &PluginMarketplace, name: String) -> Result<()> {
    marketplace.set_plugin_enabled(&name, false)?;
    Ok(())
}
