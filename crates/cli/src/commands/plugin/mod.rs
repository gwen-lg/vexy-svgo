// this_file: crates/cli/src/commands/plugin/mod.rs

pub mod search;
pub mod info;
pub mod install;
pub mod list;
pub mod update;
pub mod remove;
pub mod enable;
pub mod disable;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context, Result};
use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use tokio::time::Duration;

use crate::config::CliConfig;

/// Plugin management commands
#[derive(Debug, Args)]
pub struct PluginCommand {
    #[command(subcommand)]
    pub action: PluginAction,
}

/// Plugin subcommands
#[derive(Debug, Subcommand)]
pub enum PluginAction {
    /// Search for plugins in the marketplace
    Search {
        /// Search query
        query: String,
        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,
        /// Filter by category
        #[arg(long)]
        category: Option<String>,
        /// Maximum number of results
        #[arg(long, default_value = "20")]
        limit: usize,
    },
    /// Get detailed information about a plugin
    Info {
        /// Plugin name
        name: String,
        /// Show specific version info
        #[arg(long)]
        version: Option<String>,
    },
    /// Install a plugin
    Install {
        /// Plugin name or source
        source: String,
        /// Specific version to install
        #[arg(long)]
        version: Option<String>,
        /// Force reinstall if already installed
        #[arg(long)]
        force: bool,
        /// Install without prompting for confirmation
        #[arg(long)]
        yes: bool,
    },
    /// List installed plugins
    List {
        /// Show only outdated plugins
        #[arg(long)]
        outdated: bool,
        /// Show detailed information
        #[arg(long)]
        verbose: bool,
    },
    /// Update installed plugins
    Update {
        /// Plugin name (updates all if not specified)
        plugin: Option<String>,
        /// Update all plugins
        #[arg(long)]
        all: bool,
        /// DonPROTECTED_65_t prompt for confirmation
        #[arg(long)]
        yes: bool,
    },
    /// Enable a plugin
    Enable {
        /// Plugin name
        name: String,
    },
    /// Disable a plugin
    Disable {
        /// Plugin name
        name: String,
    },
    /// Publish a plugin to the marketplace
    Publish {
        /// Path to plugin directory
        #[arg(long, default_value = ".")]
        path: PathBuf,
        /// Perform a dry run without actually publishing
        #[arg(long)]
        dry_run: bool,
        /// API token for authentication
        #[arg(long)]
        token: Option<String>,
    },
    /// Log in to the plugin marketplace
    Login {
        /// API token
        #[arg(long)]
        token: String,
    },
    /// Log out from the plugin marketplace
    Logout,
    /// Manage plugin registries
    Registry {
        #[command(subcommand)]
        action: RegistryAction,
    },
}

/// Registry management subcommands
#[derive(Debug, Subcommand)]
pub enum RegistryAction {
    /// Add a new registry
    Add {
        /// Registry name
        name: String,
        /// Registry URL
        url: String,
    },
    /// Remove a registry
    Remove {
        /// Registry name
        name: String,
    },
    /// List configured registries
    List,
    /// Set default registry
    Default {
        /// Registry name
        name: String,
    },
}

/// Plugin metadata from marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: PluginAuthor,
    pub license: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub svgn_version: String,
    pub dependencies: HashMap<String, String>,
    pub published_at: String,
    pub downloads: u64,
    pub stars: u32,
    pub rating: Option<f32>,
    pub review_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginAuthor {
    pub name: String,
    pub email: Option<String>,
    pub github: Option<String>,
}

/// Installed plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPlugin {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub installed_at: String,
    pub config: Option<serde_json::Value>,
}

/// Plugin registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRegistry {
    pub name: String,
    pub url: String,
    pub default: bool,
}

/// Plugin marketplace client
pub struct PluginMarketplace {
    client: Client,
    config: CliConfig,
}

impl PluginMarketplace {
    /// Create a new marketplace client
    pub fn new(config: CliConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(format!("svgn-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Search for plugins
    pub async fn search_plugins(
        &self,
        query: &str,
        tag: Option<&str>,
        category: Option<&str>,
        limit: usize,
    ) -> Result<Vec<PluginInfo>> {
        let registry_url = self.get_default_registry_url()?;
        let mut url = format!("{}/api/v1/plugins", registry_url);
        
        let mut params = vec![
            ("q", query.to_string()),
            ("limit", limit.to_string()),
        ];
        
        if let Some(tag) = tag {
            params.push(("tag", tag.to_string()));
        }
        
        if let Some(category) = category {
            params.push(("category", category.to_string()));
        }
        
        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        if !query_string.is_empty() {
            url.push('?');
            url.push_str(&query_string);
        }

        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch plugin search results")?;

        if !response.status().is_success() {
            return Err(anyhow!("Search request failed: {}", response.status()));
        }

        let plugins: Vec<PluginInfo> = response
            .json()
            .await
            .context("Failed to parse search results")?;

        Ok(plugins)
    }

    /// Get plugin information
    pub async fn get_plugin_info(&self, name: &str, version: Option<&str>) -> Result<PluginInfo> {
        let registry_url = self.get_default_registry_url()?;
        let url = if let Some(version) = version {
            format!("{}/api/v1/plugins/{}/versions/{}", registry_url, name, version)
        } else {
            format!("{}/api/v1/plugins/{}", registry_url, name)
        };

        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch plugin information")?;

        if response.status().is_client_error() {
            return Err(anyhow!("Plugin '{}' not found", name));
        }

        if !response.status().is_success() {
            return Err(anyhow!("Request failed: {}", response.status()));
        }

        let plugin: PluginInfo = response
            .json()
            .await
            .context("Failed to parse plugin information")?;

        Ok(plugin)
    }

    /// Download plugin package
    pub async fn download_plugin(&self, name: &str, version: &str) -> Result<Vec<u8>> {
        let registry_url = self.get_default_registry_url()?;
        let url = format!("{}/api/v1/plugins/{}/download/{}", registry_url, name, version);

        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to download plugin package")?;

        if !response.status().is_success() {
            return Err(anyhow!("Download failed: {}", response.status()));
        }

        let bytes = response
            .bytes()
            .await
            .context("Failed to read plugin package")?;

        Ok(bytes.to_vec())
    }

    /// Publish a plugin
    pub async fn publish_plugin(&self, plugin_path: &Path, token: &str, dry_run: bool) -> Result<()> {
        // Load plugin metadata
        let metadata_path = plugin_path.join("Cargo.toml");
        if !metadata_path.exists() {
            return Err(anyhow!("No Cargo.toml found in plugin directory"));
        }

        // Create plugin package
        let package_data = self.create_plugin_package(plugin_path)?;
        
        if dry_run {
            println!("Dry run: Plugin package created successfully ({} bytes)", package_data.len());
            return Ok(());
        }

        let registry_url = self.get_default_registry_url()?;
        let url = format!("{}/api/v1/plugins", registry_url);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/octet-stream")
            .body(package_data)
            .send()
            .await
            .context("Failed to publish plugin")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Publish failed: {}", error_text));
        }

        println!("Plugin published successfully!");
        Ok(())
    }

    /// Install a plugin
    pub async fn install_plugin(&self, source: &str, version: Option<&str>, force: bool) -> Result<()> {
        // Determine if source is a plugin name or URL
        let plugin_info = if source.starts_with("git+") || source.starts_with("http") {
            // TODO: Handle git and HTTP sources
            return Err(anyhow!("Git and HTTP sources not yet implemented"));
        } else if source.contains('/') && fs::metadata(source).is_ok() {
            // Local file or directory
            return self.install_local_plugin(source, force);
        } else {
            // Plugin name from registry
            self.get_plugin_info(source, version).await?
        };

        // Check if already installed
        if !force && self.is_plugin_installed(&plugin_info.name)? {
            println!("Plugin '{}' is already installed. Use --force to reinstall.", plugin_info.name);
            return Ok(());
        }

        // Download plugin package
        println!("Downloading {}@{}", plugin_info.name, plugin_info.version);
        let package_data = self.download_plugin(&plugin_info.name, &plugin_info.version).await?;

        // Extract and install
        self.extract_and_install_plugin(&plugin_info, &package_data)?;

        println!("✓ Successfully installed {}@{}", plugin_info.name, plugin_info.version);
        Ok(())
    }

    /// List installed plugins
    pub fn list_installed_plugins(&self, outdated_only: bool) -> Result<Vec<InstalledPlugin>> {
        let plugins_dir = self.get_plugins_directory()?;
        if !plugins_dir.exists() {
            return Ok(Vec::new());
        }

        let mut installed_plugins = Vec::new();

        for entry in fs::read_dir(&plugins_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let plugin_name = entry.file_name().to_string_lossy().to_string();
                
                if let Ok(plugin) = self.load_installed_plugin_info(&plugin_name) {
                    if !outdated_only || self.is_plugin_outdated(&plugin).unwrap_or(false) {
                        installed_plugins.push(plugin);
                    }
                }
            }
        }

        Ok(installed_plugins)
    }

    /// Remove a plugin
    pub fn remove_plugin(&self, name: &str) -> Result<()> {
        let plugin_dir = self.get_plugin_directory(name)?;
        
        if !plugin_dir.exists() {
            return Err(anyhow!("Plugin '{}' is not installed", name));
        }

        fs::remove_dir_all(&plugin_dir)
            .context("Failed to remove plugin directory")?;

        // Remove from config
        self.update_plugin_config(name, |_| None)?;

        println!("✓ Successfully removed plugin '{}'", name);
        Ok(())
    }

    /// Enable/disable a plugin
    pub fn set_plugin_enabled(&self, name: &str, enabled: bool) -> Result<()> {
        if !self.is_plugin_installed(name)? {
            return Err(anyhow!("Plugin '{}' is not installed", name));
        }

        self.update_plugin_config(name, |mut plugin| {
            plugin.enabled = enabled;
            Some(plugin)
        })?;

        let action = if enabled { "enabled" } else { "disabled" };
        println!("✓ Plugin '{}' {}", name, action);
        Ok(())
    }

    // Private helper methods

    fn get_default_registry_url(&self) -> Result<String> {
        // For now, use the default registry URL
        // In the future, this would check the config for configured registries
        Ok("https://plugins.svgn.org".to_string())
    }

    fn get_plugins_directory(&self) -> Result<PathBuf> {
        let config_dir = self.config.get_config_dir()?;
        Ok(config_dir.join("plugins"))
    }

    fn get_plugin_directory(&self, name: &str) -> Result<PathBuf> {
        let plugins_dir = self.get_plugins_directory()?;
        Ok(plugins_dir.join(name))
    }

    fn is_plugin_installed(&self, name: &str) -> Result<bool> {
        let plugin_dir = self.get_plugin_directory(name)?;
        Ok(plugin_dir.exists())
    }

    fn load_installed_plugin_info(&self, name: &str) -> Result<InstalledPlugin> {
        let plugin_dir = self.get_plugin_directory(name)?;
        let metadata_file = plugin_dir.join(".svgn-plugin.json");
        
        if !metadata_file.exists() {
            return Err(anyhow!("Plugin metadata not found"));
        }

        let metadata_content = fs::read_to_string(metadata_file)?;
        let plugin: InstalledPlugin = serde_json::from_str(&metadata_content)?;
        Ok(plugin)
    }

    fn is_plugin_outdated(&self, _plugin: &InstalledPlugin) -> Result<bool> {
        // TODO: Check against registry for newer versions
        Ok(false)
    }

    fn update_plugin_config<F>(&self, name: &str, updater: F) -> Result<()>
    where
        F: FnOnce(InstalledPlugin) -> Option<InstalledPlugin>,
    {
        let plugin_dir = self.get_plugin_directory(name)?;
        let metadata_file = plugin_dir.join(".svgn-plugin.json");
        
        if let Ok(current_plugin) = self.load_installed_plugin_info(name) {
            if let Some(updated_plugin) = updater(current_plugin) {
                let metadata_content = serde_json::to_string_pretty(&updated_plugin)?;
                fs::write(metadata_file, metadata_content)?;
            } else {
                // Remove the metadata file
                if metadata_file.exists() {
                    fs::remove_file(metadata_file)?;
                }
            }
        }

        Ok(())
    }

    fn create_plugin_package(&self, plugin_path: &Path) -> Result<Vec<u8>> {
        // TODO: Create tar.gz package with proper structure and signing
        // For now, return placeholder
        Ok(b"placeholder package".to_vec())
    }

    fn install_local_plugin(&self, source: &str, _force: bool) -> Result<()> {
        // TODO: Install from local directory or archive
        println!("Installing local plugin from: {}", source);
        Ok(())
    }

    fn extract_and_install_plugin(&self, plugin_info: &PluginInfo, _package_data: &[u8]) -> Result<()> {
        // TODO: Extract package and install files
        let plugin_dir = self.get_plugin_directory(&plugin_info.name)?;
        fs::create_dir_all(&plugin_dir)?;

        // Create metadata file
        let installed_plugin = InstalledPlugin {
            name: plugin_info.name.clone(),
            version: plugin_info.version.clone(),
            enabled: true,
            installed_at: chrono::Utc::now().to_rfc3339(),
            config: None,
        };

        let metadata_file = plugin_dir.join(".svgn-plugin.json");
        let metadata_content = serde_json::to_string_pretty(&installed_plugin)?;
        fs::write(metadata_file, metadata_content)?;

        Ok(())
    }
}

/// Execute plugin command
pub async fn execute_plugin_command(command: PluginCommand, config: CliConfig) -> Result<()> {
    let marketplace = PluginMarketplace::new(config);

    match command.action {
        PluginAction::Search { query, tag, category, limit } => {
            search::search_plugin_command(&marketplace, query, tag, category, limit).await?;
        }

        PluginAction::Info { name, version } => {
            info::info_plugin_command(&marketplace, name, version).await?;
        }

        PluginAction::Install { source, version, force, yes } => {
            install::install_plugin_command(&marketplace, source, version, force, yes).await?;
        }

        PluginAction::List { outdated, verbose } => {
            list::list_plugin_command(&marketplace, outdated, verbose)?;
        }

        PluginAction::Update { plugin, all, yes } => {
            update::update_plugin_command(&marketplace, plugin, all, yes).await?;
        }

        PluginAction::Remove { name, yes } => {
            remove::remove_plugin_command(&marketplace, name, yes)?;
        }

        PluginAction::Enable { name } => {
            enable::enable_plugin_command(&marketplace, name).await?;
        }

        PluginAction::Disable { name } => {
            disable::disable_plugin_command(&marketplace, name).await?;
        }

        PluginAction::Publish { path, dry_run, token } => {
            let token = if let Some(token) = token {
                token
            } else {
                return Err(anyhow!("API token required for publishing. Use --token or run 'svgn plugin login' first."));
            };

            marketplace.publish_plugin(&path, &token, dry_run).await?;
        }

        PluginAction::Login { token } => {
            // TODO: Store token securely
            println!("Logged in successfully!");
        }

        PluginAction::Logout => {
            // TODO: Remove stored token
            println!("Logged out successfully!");
        }

        PluginAction::Registry { action } => {
            match action {
                RegistryAction::Add { name, url } => {
                    println!("Added registry '{}': {}", name, url);
                    // TODO: Add registry to config
                }
                RegistryAction::Remove { name } => {
                    println!("Removed registry '{}'", name);
                    // TODO: Remove registry from config
                }
                RegistryAction::List => {
                    println!("Configured registries:");
                    println!("  default: https://plugins.svgn.org");
                    // TODO: List registries from config
                }
                RegistryAction::Default { name } => {
                    println!("Set '{}' as default registry", name);
                    // TODO: Set default registry in config
                }
            }
        }
    }

    Ok(())
}
                    // TODO: Set default registry in config
                }
            }
        }
    }

    Ok(())
}