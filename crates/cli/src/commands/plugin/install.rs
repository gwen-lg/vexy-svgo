// this_file: crates/cli/src/commands/plugin/install.rs

use anyhow::{anyhow, Result};
use crate::commands::plugin::PluginMarketplace;

pub async fn install_plugin_command(marketplace: &PluginMarketplace, source: String, version: Option<String>, force: bool, yes: bool) -> Result<()> {
    if !yes {
        println!("Installing plugin: {}", source);
        if let Some(version) = &version {
            println!("Version: {}", version);
        }
        print!("Continue? [y/N]: ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Installation cancelled.");
            return Ok(());
        }
    }

    marketplace.install_plugin(&source, version.as_deref(), force).await?;
    Ok(())
}
