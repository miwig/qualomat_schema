#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc= include_str!("../README.md")]
mod generated;
pub mod json;
/// Path to a specific election in the dataset.
pub use generated::ElectionListItem;
pub use generated::{Answer, Category, Comment, ElectionOverview, Opinion, Party, Statement};

use json::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// All the data available for a single election
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Election {
    /// Metadata about the election
    pub metadata: ElectionOverview,
    /// All the answers that a party or user could give to a statement
    pub possible_answers: Vec<Answer>,
    /// Categories that statements are sorted into (2003-2007 only)
    pub categories: Vec<Category>,
    /// Reasons or comments given by parties to explain their opinions
    pub comments: Vec<Comment>,
    /// Opinions given by parties concerning statements
    pub opinions: Vec<Opinion>,
    /// Parties that participated in Wahl-O-Mat for this election
    pub parties: Vec<Party>,
    /// Statements that parties and users were asked to opine on
    pub statements: Vec<Statement>,
}

impl Election {
    /// Reads a single election dataset from the folder at `path`
    pub fn read_from_path(path: impl AsRef<Path>) -> Result<Self, ReadJsonError> {
        let path = path.as_ref();
        Ok(Self {
            metadata: read_json_file(path.join("overview.json"))?,
            possible_answers: read_json_file(path.join("answer.json"))?,
            categories: read_json_file(path.join("answer.json")).unwrap_or_default(),
            comments: read_json_file(path.join("comment.json")).unwrap_or_default(),
            opinions: read_json_file(path.join("opinion.json"))?,
            parties: read_json_file(path.join("party.json"))?,
            statements: read_json_file(path.join("statement.json"))?,
        })
    }

    /// Loads all election data listed in `path/election.json`.
    pub fn read_all_from_path(path: impl AsRef<Path>) -> Result<Vec<Self>, ReadJsonError> {
        let path = path.as_ref();
        let index: Vec<ElectionListItem> = read_json_file(path.join("election.json"))?;
        index
            .iter()
            .map(|election| Election::read_from_path(path.join(&election.path)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_all() {
        let elections = Election::read_all_from_path("qual-o-mat-data").unwrap();
        dbg!(elections);
    }
}
