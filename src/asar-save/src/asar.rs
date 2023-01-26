use crate::vec_pair::*;
use crate::AsarArchive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AsarFile {
    #[serde(
        serialize_with = "crate::serde_utils::display_it",
        deserialize_with = "crate::serde_utils::deserialize_number_from_string"
    )]
    pub offset: u64,
    pub size: u64,
}

impl AsarFile {
    pub fn data<'a>(&self, archive: &'a AsarArchive) -> Option<&'a [u8]> {
        archive
            .data()
            .get(self.offset as usize..(self.offset + self.size) as usize)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct AsarHeader {
    #[serde(
        default,
        deserialize_with = "deserialize_vec_pair",
        serialize_with = "serialize_vec_pair"
    )]
    pub files: Vec<(String, AsarFile)>,
    pub game: Game,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_header_line: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Game {
    #[default]
    Eu4,
    Vic3,
    Hoi4,
    Imperator,
    Ck3,
}

impl Game {
    pub fn has_header_line(&self) -> bool {
        matches!(self, Game::Vic3 | Game::Imperator | Game::Ck3)
    }

    pub fn zip_order(&self) -> &'static [&'static str] {
        match self {
            Game::Eu4 => &["meta", "gamestate", "ai"],
            Game::Ck3 | Game::Vic3 | Game::Imperator => &["gamestate"],
            Game::Hoi4 => &[],
        }
    }

    pub fn zip_file_header(&self) -> &'static [u8] {
        match self {
            Game::Eu4 => b"EU4txt",
            _ => b"",
        }
    }
}

pub struct Metadata<'a> {
    pub filename: &'a str,
}
