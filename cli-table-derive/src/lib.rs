#![forbid(unsafe_code)]
#![cfg_attr(not(any(docsrs, feature = "doc")), forbid(unstable_features))]
#![deny(missing_docs)]
#![cfg_attr(any(docsrs, feature = "doc"), feature(doc_cfg))]
//! Rust crate for printing tables on command line.
//!
//! # Usage
//!
//! Add `cli-table` in your `Cargo.toms`'s `dependencies` section
//!
//! ```toml
//! [dependencies]
//! cli-table = "0.4"
//! ```
//!
//! ## Simple usage
//!
//! ```rust,ignore
//! use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
//!
//! let table = vec![
//!     vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
//!     vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
//!     vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
//! ]
//! .table()
//! .title(vec![
//!     "Name".cell().bold(true),
//!     "Age (in years)".cell().bold(true),
//! ])
//! .bold(true);
//!
//! assert!(print_stdout(table).is_ok());
//! ```
//!
//! Below is the output of the table we created just now:
//!
//! ```markdown
//! +------------+----------------+
//! | Name       | Age (in years) |  <-- This row and all the borders/separators
//! +------------+----------------+      will appear in bold
//! | Tom        |             10 |
//! +------------+----------------+
//! | Jerry      |             15 |
//! +------------+----------------+
//! | Scooby Doo |             25 |
//! +------------+----------------+
//! ```
//!
//! ## Derive macro
//!
//! `#[derive(Table)]` can also be used to print a `Vec` or slice of `struct`s as table.
//!
//! ```rust,ignore
//! use cli_table::{format::Justify, print_stdout, Table, WithTitle};
//!
//! #[derive(Table)]
//! struct User {
//!     #[table(title = "ID", justify = "Justify::Right")]
//!     id: u64,
//!     #[table(title = "First Name")]
//!     first_name: &'static str,
//!     #[table(title = "Last Name")]
//!     last_name: &'static str,
//! }
//!
//! let users = vec![
//!     User {
//!         id: 1,
//!         first_name: "Scooby",
//!         last_name: "Doo",
//!     },
//!     User {
//!         id: 2,
//!         first_name: "John",
//!         last_name: "Cena",
//!     },
//! ];
//!
//! assert!(print_stdout(users.with_title()).is_ok());
//! ```
//!
//! Below is the output of the table we created using derive macro:
//!
//! ```markdown
//! +----+------------+-----------+
//! | ID | First Name | Last Name |  <-- This row will appear in bold
//! +----+------------+-----------+
//! |  1 | Scooby     | Doo       |
//! +----+------------+-----------+
//! |  2 | John       | Cena      |
//! +----+------------+-----------+
//! ```
//!
//! ### Field attributes
//!
//! - `title` | `name`: Used to specify title of a column. Usage: `#[table(title = "Title")]`
//! - `justify`: Used to horizontally justify the contents of a column. Usage: `#[table(justify = "Justify::Right")]`
//! - `align`: Used to vertically align the contents of a column. Usage: `#[table(align = "Align::Top")]`
//! - `color`: Used to specify color of contents of a column. Usage: `#[table(color = "Color::Red")]`
//! - `bold`: Used to specify boldness of contents of a column. Usage: `#[table(bold)]`
//! - `skip`: Used to skip a field from table. Usage: `#[table(skip)]`
//!
//! For more information on configurations available on derive macro, go to `cli-table/examples/struct.rs`.
mod context;
mod table;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Table, attributes(table))]
/// Derive macro to implementing `cli_table` traits
pub fn table(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Prepare and return the output
    table::table(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}