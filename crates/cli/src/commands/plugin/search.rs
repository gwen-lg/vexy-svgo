// this_file: crates/cli/src/commands/plugin/search.rs

use anyhow::Result;
use crate::commands::plugin::{PluginInfo, PluginMarketplace};

pub async fn search_plugin_command(marketplace: &PluginMarketplace, query: String, tag: Option<String>, category: Option<String>, limit: usize) -> Result<()> {
    let plugins = marketplace.search_plugins(&query, tag.as_deref(), category.as_deref(), limit).await?;
            
    if plugins.is_empty() {
        println!("No plugins found matching the search criteria.");
        return Ok(());
    }

    println!("Found {} plugin(s):\n", plugins.len());
    for plugin in plugins {
        println!("📦 {} v{}", plugin.name, plugin.version);
        println!("   {}", plugin.description);
        println!("   👤 {} | ⭐ {} | ⬇️  {} downloads", 
            plugin.author.name, plugin.stars, plugin.downloads);
        if !plugin.keywords.is_empty() {
            println!("   🏷️  {}", plugin.keywords.join(", "));
        }
        println!();
    }
    Ok(())
}
