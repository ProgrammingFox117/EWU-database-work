use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct DatabaseArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    ///Create, Update, Show
    Sprint(SprintCommand),
    ///Create, Update, Show
    TeamReport(TeamReportCommand),
    ///Create, Update, Show
    Individual(IndividualReportCommand),
}

//start Sprint commands
#[derive(Debug, Args)]
pub struct SprintCommand {
    #[clap(subcommand)]
    pub command: SprintSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SprintSubcommand {
    /// Create a new sprint
    Create(CreateSprint),

    /// Show all Sprints
    Show,
}

#[derive(Debug, Args)]
pub struct CreateSprint {
    /// The number of the sprint to create
    pub sprint_num: i32,

    /// The sprint date to create
    pub sprint_date: NaiveDate,
}

//start TeamReport commands
#[derive(Debug, Args)]
pub struct TeamReportCommand {
    #[clap(subcommand)]
    pub command: TeamReportSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TeamReportSubcommand {
    /// Create a new TeamReports
    Create(CreateTeamReport),

    /// Update an existing TeamReports
    Update(UpdateTeamReport),
}

#[derive(Debug, Args)]
pub struct CreateTeamReport {
    pub teams: String,
    pub sprint_num: i32,
}

#[derive(Debug, Args)]
pub struct UpdateTeamReport {
    pub teams: String,
    pub sprint_num: i32,
    pub understand_easiest: String,
    pub understand_hardest: String,
    pub approach_easiest: String,
    pub approach_hardest: String,
    pub solve_easiest: String,
    pub solve_hardest: String,
    pub evaluate_easiest: String,
    pub evaluate_hardest: String,
    pub completion: i32,
    pub contact: String,
    pub comments: String,
}

//start IndividualReport commands
#[derive(Debug, Args)]
pub struct IndividualReportCommand {
    #[clap(subcommand)]
    pub command: IndividualReportSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum IndividualReportSubcommand {
    /// Create a new sprint
    Create(CreateIndividualReport),

    /// Update an existing sprint
    Update(UpdateIndividualReport),
}

#[derive(Debug, Args)]
pub struct CreateIndividualReport {
    pub ouath_id: String,
    pub sprint_num: i32,
}

#[derive(Debug, Args)]
pub struct UpdateIndividualReport {
    pub ouath_id: String,
    pub sprint_num: i32,
    pub monday_time: i32,
    pub tuesday_time: i32,
    pub wednesday_time: i32,
    pub thursday_time: i32,
    pub friday_time: i32,
    pub saturday_time: i32,
    pub sunday_time: i32,
    pub discrepancy: String,
    pub request: String,
}
