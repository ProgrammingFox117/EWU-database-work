#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;
mod db;
mod args;
mod ops;

use ops::sprint_ops::handle_sprint_command;

use args::EntityType;
use args::DatabaseArgs;
use clap::Parser;


fn main() {
    let args = DatabaseArgs::parse();

    match args.entity_type {
        /// if "this" matches "that"
        EntityType::Sprint(sprint) => handle_sprint_command(sprint),

        /// error handling for no match command, TODO remove later when done
        _ => ()
    }
}