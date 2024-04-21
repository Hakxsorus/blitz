mod blacklist;
mod paths;
mod detector;

use std::error::Error;
use std::io::Write;
use colored::Colorize;
use prettytable::{row, Table};
use crate::detector::ScanInfo;

const SCAN_COMMAND: &str = "scan";
const EDIT_COMMAND: &str = "edit";

macro_rules! with_vertical_padding {
    ($code:block) => {{
        println!();
        $code
        println!();
    }};
}

#[tokio::main]
async fn main() {
    // Initialise the necessary files if it's not already been done.
    paths::create_app_dir().unwrap();
    paths::create_init_file_if_not_exists().unwrap();
    paths::create_blacklist_file_if_not_exists().unwrap();
    paths::download_rten_models().await.unwrap();

    // Parse the command line arguments.
    let args: Vec<String> = std::env::args().collect();
    let commands: Vec<&str> = args.iter().map(|s| s.as_str()).skip(1).collect();
    // Match the arguments against the commands.
    match commands.as_slice() {
        [SCAN_COMMAND] => {
            let scans = detector::scan().expect("Unable to scan RISK application.");
            let mut table = Table::new();

            let similar_scans: Vec<_> = scans
                .iter()
                .filter(|s| s.similarity >= 70)
                .collect();

            if similar_scans.len() == 0 {
                with_vertical_padding!({
                    println!("{}", "No Morons Here (✿◠‿◠)".cyan())
                });
                return;
            }

            table.add_row(row!["┌П┐(►˛◄’!)", "Username", "Probability"]);

            for similar_scan in similar_scans {
                let display_moron = "POSSIBLE MORON".red().bold().underline();
                let display_username = similar_scan.username.bright_cyan();
                let display_match_score = format!("{}%", similar_scan.similarity.to_string().underline());
                table.add_row(row![display_moron, display_username, display_match_score]);
            }

            with_vertical_padding!({
                table.printstd()
            });
        },
        [EDIT_COMMAND] => {
            let blacklist_path = paths::blacklist_path().unwrap();
            open::that(&blacklist_path).unwrap();
        }
        _ => eprintln!("Please enter a valid command."),
    }
}

// TODO: Use this somewhere pretty.
/// ASCII displayed when morons are detected.
const MORONS: &str = r#"
  __  __    U  ___ u   ____    U  ___ u  _   _    ____
U|' \/ '|u   \/"_ \/U |  _"\ u  \/"_ \/ | \ |"|  / __"| u
\| |\/| |/   | | | | \| |_) |/  | | | |<|  \| |><\___ \/
 | |  | |.-,_| |_| |  |  _ <.-,_| |_| |U| |\  |u u___) |
 |_|  |_| \_)-\___/   |_| \_\\_)-\___/  |_| \_|  |____/>>
<<,-,,-.       \\     //   \\_    \\    ||   \\,-.)(  (__)
 (./  \.)     (__)   (__)  (__)  (__)   (_")  (_/(__)"#;
