use std::{env, fs::File, path::PathBuf};

use anyhow::Context;
use schemars::schema::Schema;
use serde_json::{self};
use typify::TypeSpace;

fn main() -> anyhow::Result<()> {
    let mut type_space = TypeSpace::default();

    let schema_path = PathBuf::from(
        env::args()
            .nth(1)
            .context("pass qual-o-mat-data folder as first argument")?,
    )
    .join("schema");
    for file in std::fs::read_dir(&schema_path)? {
        let file = file?;
        let file_path = file.path();
        let stem = file_path
            .file_stem()
            .context("no filename")?
            .to_str()
            .context("invalid unicode")?;

        if ["list.schema"].contains(&stem) {
            continue;
        }

        let f = File::open(&file.path())?;
        let schema: Schema = serde_json::from_reader(f)?;
        type_space.add_type(&schema)?;
    }

    let mut contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};\nuse std::path::PathBuf;",
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
    )
    .replace("path: ElectionListItemPath", "path: PathBuf")
    //.replace("ElectionListItem", "Election")
    .replace("AnswerListItem", "Answer")
    .replace("CategoryListItem", "Category")
    .replace("CommentListItem", "Comment")
    .replace("OpinionListItem", "Opinion")
    .replace("PartyListItem", "Party")
    .replace("StatementListItem", "Statement")
    .replace("path: ElectionListItemPath", "path: PathBuf");

    let start = contents
        .find("\n///The path to the data directory of an election")
        .context("couldn't find start of removed section")?;
    let end = contents
        .find("///Overview and info of an election")
        .context("couldn't find end of removed section")?;

    contents.replace_range(start..end, "");

    println!("//! **DO NOT MODIFY BY HAND!**  ");
    println!("//! Generated automatically by qualomat_schema_gen.");

    println!("{}", contents);

    Ok(())
}
