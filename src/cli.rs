use crate::{GameType, PoELauncherConfig};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// do the thing
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// list launchers
    List,
    /// add new launcher
    Add {
        name: String,
        path: PathBuf,
        args: Vec<String>,
    },
    /// remove launcher by index
    Remove { index: usize },
    /// set game path (or steam)
    Game { path: String },
    /// run programs & game
    Run,
}

pub fn parse_args(cfg: &mut PoELauncherConfig) {
    let args = Cli::parse();
    match args.command {
        Commands::List => {
            println!("Launcher Items");
            for (idx, item) in cfg.launcher.iter().enumerate() {
                println!("[{:?}] {}", idx, item.name);
            }
        }
        Commands::Remove { index } => {
            if let Some(item) = cfg.launcher.get(index) {
                println!("removed launcher: {}", item.name);
                cfg.remove(index);
            } else {
                println!("launcher not found");
            }
        }
        Commands::Add { name, path, args } => {
            println!("added launcher: {}", &name);
            cfg.add(name, path, args);
        }
        Commands::Game { path } => {
            if path == "steam" {
                cfg.set_game(GameType::Steam)
            } else {
                let pbuf = PathBuf::from(path);
                if pbuf.exists() {
                    cfg.set_game(GameType::Standalone(pbuf))
                } else {
                    println!("invalid game path")
                }
            }
        }
        Commands::Run => {
            cfg.launch_tools();
            cfg.launch_game();
        }
    }
}
