use std::error::Error;
use std::fmt::format;
use std::io::Write;
use std::path::PathBuf;
use crate::blacklist::{Blacklist, Moron};
use crate::models;

const APP_DIR_NAME: &str = "risk-tracker";
const BLACKLIST_FILE_NAME: &str = "blacklist.json";
const RISK_APPLICATION_TITLE: &str = "RISK";
const SCRSHOT_FILE_NAME: &str = "players.png";
const PLAYER_SCRSHOT_FILE_NAME: &str = "crop-{}.png";

pub fn app_dir_path() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Unable to determine home directory.")?;
    Ok(home_dir.join(APP_DIR_NAME))
}

pub(crate) fn blacklist_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(app_dir_path()?.join(BLACKLIST_FILE_NAME))
}

pub(crate) fn scrshot_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(app_dir_path()?.join(SCRSHOT_FILE_NAME))
}

pub(crate) fn player_scrshot_path(n: &u32) -> Result<PathBuf, Box<dyn Error>> {
    let filename = PLAYER_SCRSHOT_FILE_NAME.replace("{}", &n.to_string());
    Ok(app_dir_path()?.join(filename))
}

pub(crate) fn detection_model_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(app_dir_path()?.join(models::DETECTION_MODEL_FILE_NAME))
}

pub(crate) fn recognition_model_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(app_dir_path()?.join(models::RECOGNITION_MODEL_FILE_NAME))
}

pub fn create_app_dir_and_blacklist_file() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(app_dir_path()?)?;
    let blacklist_path = blacklist_path()?;
    if !blacklist_path.exists() {
        let default_blacklist = default_blacklist(&blacklist_path);
        create_blacklist_file(&default_blacklist);
    }
    Ok(())
}

fn default_blacklist(blacklist_path: &PathBuf) -> Blacklist {
    let mut default_blacklist = Blacklist::new(blacklist_path);

    // Add default moron entry
    default_blacklist.morons.push(Moron {
        username: "Copy and paste this block { } to add extra entries".to_string(),
        reason: "Don't forget the comma after the }".to_string(),
    });

    default_blacklist
}

fn create_blacklist_file(default_blacklist: &Blacklist) {
    let default_blacklist_json = serde_json::to_string_pretty(&default_blacklist).unwrap();
    let mut blacklist_file = std::fs::File::create(&blacklist_path().unwrap()).unwrap();
    blacklist_file.write_all(default_blacklist_json.as_ref()).unwrap();
}