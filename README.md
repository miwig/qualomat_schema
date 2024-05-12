# qualomat_schema

[![crates.io](https://img.shields.io/crates/v/qualomat_schema.svg)](https://crates.io/crates/qualomat_schema)
[![Docs](https://docs.rs/qualomat_schema/badge.svg)](https://docs.rs/qualomat_schema)

A Rust crate for reading data from the [qual-o-mat-data](https://github.com/gockelhahn/qual-o-mat-data) repository, which is an [open data](https://en.wikipedia.org/wiki/Open_data) archive for the german voting advice tool [_Wahl-O-Mat_](https://www.wahl-o-mat.de).

## Usage

Clone the data repository:

```bash
git clone https://github.com/gockelhahn/qual-o-mat-data
```

Read the data for all elections:

```rust
let elections = Election::read_all_from_path("qual-o-mat-data")?;
```

## Code generation

The generated code was created from the [Qual-O-Mat schema](https://github.com/gockelhahn/qual-o-mat-data/tree/16babd5b0650e2e2f5acf6f5b76816ce09969f67/schema) using [typify](https://crates.io/crates/typify) and the code in [qualomat_schema_gen](https://github.com/miwig/qualomat_schema/tree/main/qualomat_schema_gen/src/main.rs).
