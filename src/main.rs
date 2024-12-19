mod apis;
mod config;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

use crate::apis::mimecast::MimecastClient;
use crate::config::{Config, GraphConfig, MimecastConfig};

#[derive(Parser)]
#[command(name = "cli_tool")]
#[command(about = "A CLI tool for REST API interactions and shell commands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure API settings
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Mimecast API commands
    Mimecast {
        #[command(subcommand)]
        command: MimecastCommands,
    },
    /// Execute a shell command
    Exec {
        /// The command to execute
        command: String,
        /// Optional arguments for the command
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Configure Mimecast API settings
    Mimecast {
        /// Base URL for the API (e.g., https://eu-api.mimecast.com)
        #[arg(long)]
        base_url: String,
        /// Application ID from Mimecast Administration Console
        #[arg(long)]
        app_id: String,
        /// Application Key from Mimecast Administration Console
        #[arg(long)]
        app_key: String,
    },
    /// Configure Microsoft Graph API settings
    Graph {
        /// Application (client) ID from Azure AD app registration
        #[arg(long)]
        client_id: String,
        /// Client secret (optional, for daemon/service apps)
        #[arg(long)]
        client_secret: Option<String>,
    },
}

#[derive(Subcommand)]
enum MimecastCommands {
    /// Track a message
    Track {
        /// Search query (e.g., subject:"Test Email" or from:user@example.com)
        query: String,
    },
    /// Manage TTP URL Protection
    Url {
        #[command(subcommand)]
        command: UrlCommands,
    },
    /// Manage configuration snapshots
    Snapshot {
        #[command(subcommand)]
        command: SnapshotCommands,
    },
}

#[derive(Subcommand)]
enum SnapshotCommands {
    /// Create a new configuration snapshot
    Create {
        /// Description of the snapshot
        description: String,
    },
    /// List configuration snapshots
    List {
        /// Start position for pagination
        #[arg(long)]
        start: Option<String>,
        /// Maximum number of items to return
        #[arg(long)]
        limit: Option<i32>,
    },
    /// Restore a configuration snapshot
    Restore {
        /// ID of the snapshot to restore
        id: String,
    },
    /// Export a configuration snapshot
    Export {
        /// ID of the snapshot to export
        id: String,
    },
}

#[derive(Subcommand)]
enum UrlCommands {
    /// Get information about a managed URL
    Get {
        /// The URL to look up
        url: String,
    },
    /// Create a managed URL
    Create {
        /// The URL to manage
        url: String,
        /// Action to take (block or permit)
        action: String,
        /// Optional comment
        #[arg(long)]
        comment: Option<String>,
    },
}

fn execute_shell_command(command: &str, args: &[String]) -> Result<()> {
    let output = Command::new(command)
        .args(args)
        .output()
        .context("Failed to execute command")?;

    if !output.status.success() {
        println!("Command failed with error:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    match cli.command {
        Commands::Config { command } => match command {
            ConfigCommands::Mimecast {
                base_url,
                app_id,
                app_key,
            } => {
                config.mimecast = Some(MimecastConfig {
                    base_url,
                    app_id,
                    app_key,
                });
                config.save()?;
                println!("Mimecast configuration updated successfully");
            }
            ConfigCommands::Graph {
                client_id,
                client_secret,
            } => {
                config.msgraph = Some(GraphConfig {
                    client_id,
                    client_secret,
                });
                config.save()?;
                println!("Microsoft Graph configuration updated successfully");
            }
        },
        Commands::Mimecast { command } => {
            let mimecast_config = config
                .mimecast
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Mimecast is not configured. Use 'config mimecast' first."))?;
            
            let mut client = MimecastClient::new(mimecast_config.clone())?;

            match command {
                MimecastCommands::Track { query } => {
                    let result = client.track_message(&query).await?;
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                MimecastCommands::Url { command } => match command {
                    UrlCommands::Get { url } => {
                        let result = client.get_managed_url(&url).await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                    UrlCommands::Create { url, action, comment } => {
                        let result = client.create_managed_url(&url, &action, comment).await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                },
                MimecastCommands::Snapshot { command } => match command {
                    SnapshotCommands::Create { description } => {
                        let result = client.create_snapshot(&description).await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                    SnapshotCommands::List { start, limit } => {
                        let result = client.list_snapshots(start, limit).await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                    SnapshotCommands::Restore { id } => {
                        let result = client.restore_snapshot(&id).await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                    SnapshotCommands::Export { id } => {
                        let result = client.export_snapshot(&id).await?;
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                },
            }
        }
        Commands::Exec { command, args } => {
            execute_shell_command(&command, &args)?;
        }
    }

    Ok(())
}
