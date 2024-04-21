mod blacklist;
mod paths;
mod detector;

use std::error::Error;
use std::io::Write;
use colored::Colorize;
use prettytable::{row, Table};

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
    // Initialise the application if this is the first time running.
    paths::create_app_dir_and_blacklist_file().unwrap();
    paths::download_rten_models().await.unwrap();
    // Parse the command line arguments.
    let args: Vec<String> = std::env::args().collect();
    let commands: Vec<&str> = args.iter().map(|s| s.as_str()).skip(1).collect();
    // Match the arguments against the commands.
    match commands.as_slice() {
        [SCAN_COMMAND] => {
            let mut scans = detector::scan().expect("Unable to scan the RISK application.");
            let mut table = Table::new();

            if scans.len() == 0 {
                println!("{}", "No Morons Here (✿◠‿◠)".cyan());
                return;
            }

            table.add_row(row!["┌П┐(►˛◄’!)", "Username", "Probability"]);
            scans.sort_by_key(|s| std::cmp::Reverse(s.similarity));
            for scan_info in &scans {
                if scan_info.similarity <= 70 {
                    continue;
                }

                let display_moron =  "POSSIBLE MORON".red().bold().underline();
                let display_username = scan_info.username.bright_cyan();
                let display_match_score = format!("{}%", scan_info.similarity.to_string().underline());
                table.add_row(row![display_moron, display_username, display_match_score]);
            }

            with_vertical_padding!({table.printstd()})
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
