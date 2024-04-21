use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Blacklist {
    path_buf: PathBuf,
    pub morons: Vec<Moron>,
}

#[derive(Serialize, Deserialize)]
pub struct Moron {
    pub username: String,
    pub reason: String
}

impl Blacklist {
    pub fn new(path_buf: &PathBuf) -> Self {
        Blacklist {
            path_buf: path_buf.clone(),
            morons: vec![]
        }
    }
    pub fn load(path_buf: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Self::read_and_deserialize(&path_buf)
    }

    fn read_and_deserialize(path_buf: &PathBuf) -> Result<Blacklist, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path_buf)?;
        let blacklist: Blacklist = serde_json::from_str(&content)?;
        Ok(blacklist)
    }
}