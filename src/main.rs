use std::fs;
use std::path::Path;
use std::str::FromStr;
use clap::Parser;
use crate::args::Args;
use crate::config::Config;
use crate::profile::Profile;
use crate::error::AppError;

mod args;
mod config;
mod profile;
mod error;
mod git;

fn main() {
    let args = Args::parse();
    if let Err(e) = handle_args(args) {
        println!("Error: {e}");
        std::process::exit(1);
    }
}

fn handle_args(args: Args) -> Result<(), AppError> {
    let mut config = get_or_create_config(&args.config)?;
    match args.command {
        args::Command::Add { name, email, profile, signing_key, ssh_command } => {
            config.insert(profile.as_deref().unwrap_or(&name), &Profile::new(&name, &email, signing_key, ssh_command));
            save_config(&args.config, &config)
        },
        args::Command::Delete { profile } => {
            config.remove(&profile);
            save_config(&args.config, &config)
        },
        args::Command::Export => {
            println!("{config}");
            Ok(())
        },
        args::Command::List => {
            for (key, value) in config.iter() {
                println!("{key}");
                println!("  Name: {}", value.name);
                println!("  Email: {}", value.email);
                if let Some(signing_key) = &value.signing_key {
                    println!("  Signing Key: {signing_key}");
                }
                if let Some(ssh_command) = &value.ssh_command {
                    println!("  SSH Command: {ssh_command}");
                }
                println!();
            }
            Ok(())
        },
        args::Command::Use { profile, repo } => {
            let config = config.get(&profile).ok_or(AppError::Config(format!("User '{profile}' not found.")))?;
            git::update_config(&repo, config)
        }
    }
}

fn save_config(path: &str, config: &Config) -> Result<(), AppError> {
    let expanded = shellexpand::tilde(path);
    let path = Path::new(expanded.as_ref());
    fs::write(path, format!("{config}")).map_err(|e| AppError::File(e.to_string()))
}

fn get_or_create_config(path: &str) -> Result<Config, AppError> {
    let expanded = shellexpand::tilde(path);
    let path = Path::new(expanded.as_ref());
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::File(e.to_string()))?;
    }

    if !path.exists() {
        let result = Config::default();
        fs::write(path, format!("{result}")).map_err(|e| AppError::File(e.to_string()))?;
        Ok(result)
    } else {
        let content = fs::read_to_string(path).map_err(|e| AppError::File(e.to_string()))?;
        Config::from_str(content.as_str())
    }
}
