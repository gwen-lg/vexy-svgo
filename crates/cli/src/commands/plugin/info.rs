// this_file: crates/cli/src/commands/plugin/info.rs

use crate::commands::plugin::{PluginInfo, PluginMarketplace};
use anyhow::Result;

pub async fn info_plugin_command(
    marketplace: &PluginMarketplace,
    name: String,
    version: Option<String>,
) -> Result<()> {
    let plugin = marketplace
        .get_plugin_info(&name, version.as_deref())
        .await?;

    println!("📦 {} v{}", plugin.name, plugin.version);
    println!("📝 {}", plugin.description);
    println!("👤 Author: {}", plugin.author.name);
    if let Some(email) = &plugin.author.email {
        println!("✉️  Email: {}", email);
    }
    println!("📄 License: {}", plugin.license);
    println!(
        "⭐ Stars: {} | ⬇️ Downloads: {}",
        plugin.stars, plugin.downloads
    );
    if let Some(rating) = plugin.rating {
        println!(
            "⭐ Rating: {:.1}/5.0 ({} reviews)",
            rating, plugin.review_count
        );
    }
    println!("🔧 Vexy SVGO Version: {}", plugin.vexy_svgo_version);

    if !plugin.keywords.is_empty() {
        println!("🏷️  Keywords: {}", plugin.keywords.join(", "));
    }

    if !plugin.categories.is_empty() {
        println!("📂 Categories: {}", plugin.categories.join(", "));
    }

    if let Some(homepage) = &plugin.homepage {
        println!("🏠 Homepage: {}", homepage);
    }

    if let Some(repository) = &plugin.repository {
        println!("📂 Repository: {}", repository);
    }

    if !plugin.dependencies.is_empty() {
        println!("\n📦 Dependencies:");
        for (name, version) in &plugin.dependencies {
            println!("   {} = \"{}\"", name, version);
        }
    }
    Ok(())
}
