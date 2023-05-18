use chrono::NaiveDate;
use clap::{
    Args,
    Parser,
    Subcommand
};
use diesel::sql_types::Double;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct DatabaseArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    ///Create, Update, Show
    Sprint(SprintCommand),
    ///Create, Update, show
    TeamReport(TeamReportCommand),
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

    /// Update an existing sprint
    Update(UpdateSprint),

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

#[derive(Debug, Args)]
pub struct UpdateSprint {
    /// The update of sprint number
    pub sprint_num: i32,

    /// The update of sprint date
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

    /// Show all TeamReports
    Show,
}

#[derive(Debug, Args)]
pub struct CreateTeamReport {
    pub teams: String,
    pub sprint_num: i32,
    pub understand_easiest: String,
    pub understand_hardest: String,
    pub approach_easiest:   String,
    pub approach_hardest:   String,
    pub solve_easiest:      String,
    pub solve_hardest:      String,
    pub evaluate_easiest:   String,
    pub evaluate_hardest:   String,
    pub completion:         f32,
    pub contact:            String,
    pub comments:           String,
}

#[derive(Debug, Args)]
pub struct UpdateTeamReport {
    pub sprint_num: i32,
    pub understand_easiest: String,
    pub understand_hardest: String,
    pub approach_easiest:   String,
    pub approach_hardest:   String,
    pub solve_easiest:      String,
    pub solve_hardest:      String,
    pub evaluate_easiest:   String,
    pub evaluate_hardest:   String,
    pub completion:         f32,
    pub contact:            String,
    pub comments:           String,
}