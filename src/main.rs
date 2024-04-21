mod blacklist;
mod paths;
mod models;
mod detector;

use std::error::Error;
use std::io::Write;
use colored::Colorize;
use prettytable::{row, Table};
use rten_tensor::AsView;

// Command Constants
const HELP_COMMAND: &str = "help";
const SCAN_COMMAND: &str = "scan";
const EDIT_COMMAND: &str = "edit";
const PLAYERS_COMMAND: &str = "players";
const COUNTRIES_COMMAND: &str = "countries";

// Main Function
#[tokio::main]
async fn main() {
    paths::create_app_dir_and_blacklist_file().unwrap();
    models::download_rten_models().await.unwrap();

    let args: Vec<String> = std::env::args().collect();
    let commands: Vec<&str> = args.iter().map(|s| s.as_str()).skip(1).collect();

    match commands.as_slice() {
        [SCAN_COMMAND] => {
            let matches = detector::detect().unwrap();
            if matches.len() == 0 {
                println!("{}", "No Morons detected :)".magenta());
                return;
            }

            let mut table = Table::new();
            table.add_row(row!["Moron?", "Username", "Match Score"]);
            for match_info in matches {
                table.add_row(row!["POSSIBLE MORON".bright_red().bold().underline(), match_info.username.bright_cyan(), match_info.score.to_string().underline()]);
            }
            table.printstd();
        },
        [EDIT_COMMAND] => {
            let blacklist_path = paths::blacklist_path().unwrap();
            open::that(&blacklist_path).unwrap();
        }
        _ => eprintln!("Please enter a valid command."),
    }
}