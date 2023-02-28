pub mod cli;

use serde::{Deserialize, Serialize};
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use sysinfo::{ProcessExt, System, SystemExt};

static STEAM_APPID: usize = 238960;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LauncherItem {
    name: String,
    path: PathBuf,
    args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameType {
    Steam,
    Standalone(PathBuf),
}

impl GameType {
    fn launch(&self) {
        let mut t = System::new();
        t.refresh_processes();

        match self {
            GameType::Standalone(path) => todo!("launch game at {}", path.display()),
            GameType::Steam => {
                if let Some(p) = t.processes_by_exact_name("steam.exe").peekable().peek() {
                    Command::new(p.exe())
                        .arg(format!("steam://rungameid/{}", STEAM_APPID))
                        .spawn()
                        .expect("unable to run game on steam!")
                } else {
                    panic!("is steam running?")
                };
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoELauncherConfig {
    launcher: Vec<LauncherItem>,
    game: GameType,
}

impl Default for PoELauncherConfig {
    fn default() -> Self {
        PoELauncherConfig {
            launcher: Vec::new(),
            game: GameType::Steam,
        }
    }
}

impl PoELauncherConfig {
    fn add(&mut self, name: String, path: PathBuf, args: Vec<String>) {
        match self.launcher.iter().find(|l| l.name == name) {
            Some(l) => panic!("launcher exists: {}", l.name),
            None => {
                self.launcher.push(LauncherItem { name, path, args });
            }
        }
    }

    fn remove(&mut self, index: usize) {
        self.launcher.swap_remove(index);
    }

    fn launch_tools(&self) {
        let mut sysinfo = System::new();

        for i in &self.launcher {
            if is_process_running(&mut sysinfo, &i.path) {
                continue;
            }

            Command::new(&i.path)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .creation_flags(0x00000008)
                .args(&i.args)
                .spawn()
                .expect(&format!("failed to run {}", i.name));
        }
    }

    fn launch_game(&self) {
        self.game.launch()
    }

    fn set_game(&mut self, t: GameType) {
        match t {
            GameType::Standalone(path) => self.game = GameType::Standalone(path),
            GameType::Steam => self.game = GameType::Steam,
        }
    }
}

fn is_process_running(sysinfo: &mut System, path: &PathBuf) -> bool {
    let strpath = path.file_name().expect("invalid path");

    sysinfo.refresh_processes();
    sysinfo
        .processes_by_name(&strpath.to_string_lossy())
        .peekable()
        .peek()
        .is_some()
}

fn main() -> Result<(), std::io::Error> {
    let mut cfg: PoELauncherConfig =
        confy::load("poelauncher", None).expect("unable to load config file");
    cli::parse_args(&mut cfg);
    confy::store("poelauncher", None, &cfg).expect("unable to save config file");
    Ok(())
}
