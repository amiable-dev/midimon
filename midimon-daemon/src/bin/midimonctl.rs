// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon daemon control CLI
//!
//! Command-line interface for controlling the MIDIMon daemon.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use midimon_daemon::{IpcClient, IpcCommand, get_socket_path};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "midimonctl")]
#[command(about = "Control the MIDIMon daemon", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// JSON output format
    #[arg(short, long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Check daemon status
    Status,

    /// Reload configuration
    Reload,

    /// Stop the daemon
    Stop,

    /// Validate configuration file
    Validate {
        /// Path to config file (defaults to daemon's current config)
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Ping the daemon
    Ping,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging if verbose
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_env_filter("midimon_daemon=debug")
            .init();
    }

    match execute_command(&cli).await {
        Ok(()) => Ok(()),
        Err(e) => {
            if !cli.json {
                eprintln!("{} {}", "Error:".red().bold(), e);
            } else {
                let error_json = serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                });
                println!("{}", serde_json::to_string_pretty(&error_json)?);
            }
            std::process::exit(1);
        }
    }
}

async fn execute_command(cli: &Cli) -> Result<()> {
    // Get socket path
    let socket_path = get_socket_path()
        .context("Failed to determine IPC socket path")?
        .to_string_lossy()
        .to_string();

    // Create IPC client
    let mut client = IpcClient::new(socket_path)
        .await
        .context("Failed to connect to daemon. Is the daemon running?")?;

    // Execute command
    match &cli.command {
        Commands::Status => handle_status(&mut client, cli.json).await?,
        Commands::Reload => handle_reload(&mut client, cli.json).await?,
        Commands::Stop => handle_stop(&mut client, cli.json).await?,
        Commands::Validate { config } => handle_validate(&mut client, config, cli.json).await?,
        Commands::Ping => handle_ping(&mut client, cli.json).await?,
    }

    Ok(())
}

async fn handle_status(client: &mut IpcClient, json: bool) -> Result<()> {
    let response = client
        .send_command(IpcCommand::Status, Value::Null)
        .await
        .context("Failed to get daemon status")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        // Pretty print status
        if let Some(data) = response.data {
            println!("{}", "MIDIMon Daemon Status".bold().cyan());
            println!("{}", "─".repeat(50));

            if let Some(state) = data.get("state").and_then(|v| v.as_str()) {
                let state_colored = match state {
                    "Running" => state.green(),
                    "Reloading" => state.yellow(),
                    "Degraded" => state.red(),
                    _ => state.normal(),
                };
                println!("State:           {}", state_colored);
            }

            if let Some(mode) = data.get("current_mode").and_then(|v| v.as_str()) {
                println!("Current Mode:    {}", mode.cyan());
            }

            if let Some(config_path) = data.get("config_path").and_then(|v| v.as_str()) {
                println!("Config:          {}", config_path);
            }

            if let Some(uptime) = data.get("uptime_secs").and_then(|v| v.as_u64()) {
                println!("Uptime:          {}", format_duration(uptime));
            }

            if let Some(events) = data.get("events_processed").and_then(|v| v.as_u64()) {
                println!("Events:          {}", format_number(events));
            }

            if let Some(reloads) = data.get("config_reloads").and_then(|v| v.as_u64()) {
                println!("Config Reloads:  {}", reloads);
            }

            // Reload statistics
            if let Some(reload_stats) = data.get("reload_stats") {
                println!("\n{}", "Reload Performance".bold());
                println!("{}", "─".repeat(50));

                if let Some(last) = reload_stats.get("last_reload_ms").and_then(|v| v.as_u64()) {
                    println!("Last Reload:     {} ms", last);
                }
                if let Some(avg) = reload_stats.get("avg_reload_ms").and_then(|v| v.as_u64()) {
                    let grade = if avg < 20 {
                        "A".green()
                    } else if avg < 50 {
                        "B".yellow()
                    } else {
                        "C".red()
                    };
                    println!("Average:         {} ms (grade: {})", avg, grade);
                }
                if let Some(fastest) = reload_stats
                    .get("fastest_reload_ms")
                    .and_then(|v| v.as_u64())
                {
                    println!("Fastest:         {} ms", fastest);
                }
                if let Some(slowest) = reload_stats
                    .get("slowest_reload_ms")
                    .and_then(|v| v.as_u64())
                {
                    println!("Slowest:         {} ms", slowest);
                }
            }

            println!();
        }
    }

    Ok(())
}

async fn handle_reload(client: &mut IpcClient, json: bool) -> Result<()> {
    if !json {
        println!("Reloading configuration...");
    }

    let response = client
        .send_command(IpcCommand::Reload, Value::Null)
        .await
        .context("Failed to reload configuration")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else if let Some(data) = response.data {
        println!("{}", "✓ Configuration reloaded successfully".green());

        if let Some(duration) = data.get("reload_duration_ms").and_then(|v| v.as_u64()) {
            let grade = data
                .get("performance_grade")
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let grade_colored = match grade {
                "A" => grade.green(),
                "B" => grade.yellow(),
                _ => grade.red(),
            };
            println!("Duration:  {} ms (grade: {})", duration, grade_colored);
        }

        if let Some(modes) = data.get("modes_loaded").and_then(|v| v.as_u64()) {
            println!("Modes:     {}", modes);
        }

        if let Some(mappings) = data.get("mappings_loaded").and_then(|v| v.as_u64()) {
            println!("Mappings:  {}", mappings);
        }
    }

    Ok(())
}

async fn handle_stop(client: &mut IpcClient, json: bool) -> Result<()> {
    if !json {
        println!("Stopping daemon...");
    }

    let response = client
        .send_command(IpcCommand::Stop, Value::Null)
        .await
        .context("Failed to stop daemon")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        println!("{}", "✓ Daemon stopped successfully".green());
    }

    Ok(())
}

async fn handle_validate(
    client: &mut IpcClient,
    config_path: &Option<PathBuf>,
    json: bool,
) -> Result<()> {
    let args = if let Some(path) = config_path {
        serde_json::json!({ "path": path })
    } else {
        Value::Null
    };

    let response = client
        .send_command(IpcCommand::ValidateConfig, args)
        .await
        .context("Failed to validate configuration")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else if let Some(data) = response.data
        && let Some(valid) = data.get("valid").and_then(|v| v.as_bool())
    {
        if valid {
            println!("{}", "✓ Configuration is valid".green());

            if let Some(modes) = data.get("modes").and_then(|v| v.as_u64()) {
                println!("Modes:    {}", modes);
            }

            if let Some(mappings) = data.get("mappings").and_then(|v| v.as_u64()) {
                println!("Mappings: {}", mappings);
            }
        } else {
            println!("{}", "✗ Configuration is invalid".red());
        }
    }

    Ok(())
}

async fn handle_ping(client: &mut IpcClient, json: bool) -> Result<()> {
    let start = std::time::Instant::now();
    let response = client
        .send_command(IpcCommand::Ping, Value::Null)
        .await
        .context("Failed to ping daemon")?;

    let latency = start.elapsed();

    if json {
        let mut resp_json = serde_json::to_value(&response)?;
        if let Some(obj) = resp_json.as_object_mut() {
            obj.insert(
                "latency_ms".to_string(),
                serde_json::json!(latency.as_millis()),
            );
        }
        println!("{}", serde_json::to_string_pretty(&resp_json)?);
    } else {
        println!(
            "{} ({:.2} ms)",
            "✓ Daemon is responding".green(),
            latency.as_secs_f64() * 1000.0
        );
    }

    Ok(())
}

/// Format duration in seconds to human-readable string
fn format_duration(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Format large numbers with comma separators
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars().rev() {
        if count == 3 {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}
