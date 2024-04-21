use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Blacklist containing a list of [`Moron`].
#[derive(Serialize, Deserialize)]
pub struct Blacklist {
    /// The path to the serialised blacklist.
    path_buf: PathBuf,
    /// The list of blacklisted morons.
    pub morons: Vec<Moron>,
}

/// A blacklisted moron.
#[derive(Serialize, Deserialize)]
pub struct Moron {
    /// The moron's username.
    pub username: String,
    /// Why the moron is blacklisted.
    pub reason: String
}

impl Blacklist {
    /// Creates a new empty [`Blacklist`].
    ///
    /// # Arguments
    /// * `blacklist_path` - A reference to the [`PathBuf`] representing the path to the blacklist file.
    pub fn new(blacklist_path: &PathBuf) -> Self {
        Blacklist {
            path_buf: blacklist_path.clone(),
            morons: vec![]
        }
    }

    /// Loads and deserializes an existing [`Blacklist`] JSON file into a new [`Blacklist`].
    ///
    /// # Arguments
    /// * `blacklist_path` - A reference to the [`PathBuf`] representing the path to the blacklist file.
    pub fn load(blacklist_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(blacklist_path)?;
        let blacklist: Blacklist = serde_json::from_str(&content)?;
        Ok(blacklist)
    }
}