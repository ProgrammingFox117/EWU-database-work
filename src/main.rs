#[macro_use]
extern crate diesel;
extern crate dotenv;

mod args;
mod db;
mod models;
mod ops;
mod schema;

use ops::individual_report_ops::handle_individual_report_command;
use ops::sprint_ops::handle_sprint_command;
use ops::team_report_ops::handle_team_report_command;

use args::DatabaseArgs;
use args::EntityType;
use clap::Parser;

fn main() {
    let args = DatabaseArgs::parse();

    match args.entity_type {
        // if "this" matches "that"
        EntityType::Sprint(sprint) => handle_sprint_command(sprint),
        EntityType::TeamReport(team_report) => handle_team_report_command(team_report),
        EntityType::Individual(individual_report) => {
            handle_individual_report_command(individual_report)
        }
    }
}
