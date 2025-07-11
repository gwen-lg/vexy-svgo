// this_file: crates/cli/src/commands/plugin/list.rs

use anyhow::Result;
use crate::commands::plugin::{InstalledPlugin, PluginMarketplace};

pub fn list_plugin_command(marketplace: &PluginMarketplace, outdated_only: bool, verbose: bool) -> Result<()> {
    let plugins = marketplace.list_installed_plugins(outdated_only)?;
            
    if plugins.is_empty() {
        if outdated_only {
            println!("No outdated plugins found.");
        } else {
            println!("No plugins installed.");
        }
        return Ok(());
    }

    println!("Installed plugins:\n");
    for plugin in plugins {
        let status = if plugin.enabled { "✓" } else { "✗" };
        println!("{} {} v{}", status, plugin.name, plugin.version);
        
        if verbose {
            println!("   Installed: {}", plugin.installed_at);
            println!("   Enabled: {}", plugin.enabled);
            if plugin.config.is_some() {
                println!("   Configured: Yes");
            }
            println!();
        }
    }
    Ok(())
}