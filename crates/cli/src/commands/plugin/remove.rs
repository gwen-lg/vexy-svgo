// this_file: crates/cli/src/commands/plugin/remove.rs

use anyhow::Result;
use crate::commands::plugin::PluginMarketplace;

pub fn remove_plugin_command(marketplace: &PluginMarketplace, name: String, yes: bool) -> Result<()> {
    if !yes {
        print!("Remove plugin '{}'? [y/N]: ", name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Removal cancelled.");
            return Ok(());
        }
    }

    marketplace.remove_plugin(&name)?;
    Ok(())
}
