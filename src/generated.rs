//! **DO NOT MODIFY BY HAND!**  
//! Generated automatically by qualomat_schema_gen.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///Answer
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Answer",
///  "type": "object",
///  "required": [
///    "id",
///    "message"
///  ],
///  "properties": {
///    "id": {
///      "description": "Unique identifier",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "message": {
///      "description": "Answer text",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Answer {
    ///Unique identifier
    pub id: u64,
    ///Answer text
    pub message: String,
}
impl From<&Answer> for Answer {
    fn from(value: &Answer) -> Self {
        value.clone()
    }
}
///Category
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Category",
///  "type": "object",
///  "required": [
///    "id",
///    "label"
///  ],
///  "properties": {
///    "id": {
///      "description": "Unique identifier",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "label": {
///      "description": "Category text",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    ///Unique identifier
    pub id: u64,
    ///Category text
    pub label: String,
}
impl From<&Category> for Category {
    fn from(value: &Category) -> Self {
        value.clone()
    }
}
///Comment
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Comment",
///  "type": "object",
///  "required": [
///    "id",
///    "text"
///  ],
///  "properties": {
///    "id": {
///      "description": "Unique identifier",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "text": {
///      "description": "Comment text",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Comment {
    ///Unique identifier
    pub id: u64,
    ///Comment text
    pub text: String,
}
impl From<&Comment> for Comment {
    fn from(value: &Comment) -> Self {
        value.clone()
    }
}
///Election
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Election",
///  "type": "object",
///  "required": [
///    "date",
///    "id",
///    "name",
///    "path"
///  ],
///  "properties": {
///    "date": {
///      "description": "Date the election held place on",
///      "type": "string",
///      "format": "date"
///    },
///    "id": {
///      "description": "Unique identifier",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "name": {
///      "description": "Election name",
///      "type": "string"
///    },
///    "path": {
///      "description": "The path to the data directory of an election",
///      "type": "string",
///      "pattern": "^data/[0-9]{4}/[a-z]*$",
///      "$comment": "Current path: data/[YEAR]/location"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ElectionListItem {
    ///Date the election held place on
    pub date: chrono::naive::NaiveDate,
    ///Unique identifier
    pub id: u64,
    ///Election name
    pub name: String,
    ///The path to the data directory of an election
    pub path: PathBuf,
}
impl From<&ElectionListItem> for ElectionListItem {
    fn from(value: &ElectionListItem) -> Self {
        value.clone()
    }
}///Overview and info of an election
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Election overview",
///  "description": "Overview and info of an election",
///  "type": "object",
///  "required": [
///    "data_source",
///    "date",
///    "info",
///    "slug",
///    "title"
///  ],
///  "properties": {
///    "data_source": {
///      "description": "URL to the source of the election data",
///      "type": "string",
///      "format": "uri"
///    },
///    "date": {
///      "description": "Date the election held place on",
///      "type": "string",
///      "format": "date"
///    },
///    "info": {
///      "description": "URL to the Wikipedia page of the election",
///      "type": "string",
///      "format": "uri"
///    },
///    "slug": {
///      "description": "Context path / slug of the original Wahl-o-Mat",
///      "type": "string",
///      "format": "uri"
///    },
///    "title": {
///      "description": "Election name",
///      "type": "string"
///    }
///  },
///  "$schema": "http://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ElectionOverview {
    ///URL to the source of the election data
    pub data_source: String,
    ///Date the election held place on
    pub date: chrono::naive::NaiveDate,
    ///URL to the Wikipedia page of the election
    pub info: String,
    ///Context path / slug of the original Wahl-o-Mat
    pub slug: String,
    ///Election name
    pub title: String,
}
impl From<&ElectionOverview> for ElectionOverview {
    fn from(value: &ElectionOverview) -> Self {
        value.clone()
    }
}
///Opinion
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Opinion",
///  "type": "object",
///  "required": [
///    "answer",
///    "comment",
///    "id",
///    "party",
///    "statement"
///  ],
///  "properties": {
///    "answer": {
///      "description": "ID representing the party answer",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "comment": {
///      "description": "ID representing the party comment",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "minimum": 0.0
///    },
///    "id": {
///      "description": "Unique identifier",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "party": {
///      "description": "ID representing the party",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "statement": {
///      "description": "ID representing the statement",
///      "type": "integer",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Opinion {
    ///ID representing the party answer
    pub answer: u64,
    ///ID representing the party comment
    pub comment: Option<u64>,
    ///Unique identifier
    pub id: u64,
    ///ID representing the party
    pub party: u64,
    ///ID representing the statement
    pub statement: u64,
}
impl From<&Opinion> for Opinion {
    fn from(value: &Opinion) -> Self {
        value.clone()
    }
}
///Party
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Party",
///  "type": "object",
///  "required": [
///    "id",
///    "longname",
///    "name"
///  ],
///  "properties": {
///    "id": {
///      "description": "Unique identifier",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "longname": {
///      "description": "Full party name",
///      "type": "string"
///    },
///    "name": {
///      "description": "Short party name",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Party {
    ///Unique identifier
    pub id: u64,
    ///Full party name
    pub longname: String,
    ///Short party name
    pub name: String,
}
impl From<&Party> for Party {
    fn from(value: &Party) -> Self {
        value.clone()
    }
}
///Statement
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Statement",
///  "type": "object",
///  "required": [
///    "category",
///    "id",
///    "label",
///    "text"
///  ],
///  "properties": {
///    "category": {
///      "description": "ID representing the statement category",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "minimum": 0.0
///    },
///    "id": {
///      "description": "Unique identifier",
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "label": {
///      "description": "Short statement text",
///      "type": "string"
///    },
///    "text": {
///      "description": "Full statement text",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Statement {
    ///ID representing the statement category
    pub category: Option<u64>,
    ///Unique identifier
    pub id: u64,
    ///Short statement text
    pub label: String,
    ///Full statement text
    pub text: String,
}
impl From<&Statement> for Statement {
    fn from(value: &Statement) -> Self {
        value.clone()
    }
}

