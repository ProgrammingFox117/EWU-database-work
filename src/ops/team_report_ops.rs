use crate::args::{TeamReportSubcommand, CreateTeamReport,
                  UpdateTeamReport, TeamReportCommand};
use crate::db::establish_connection;
use crate::models::{NewTeamReport, team_report};
use diesel::prelude::*;
use diesel::sql_types::*;



pub fn handle_team_report_command(team_report_cmd: TeamReportCommand){
    let command = team_report_cmd.command;
    match command{
        TeamReportSubcommand::Create(team_report_cmd) =>{
            create_team_report(team_report_cmd);
        }
        TeamReportSubcommand::Update(team_report_cmd) =>{
            update_team_report(team_report_cmd);
        }
        TeamReportSubcommand::Show => {
            show_team_report();
        }
    }
}

pub fn create_team_report(team_report_cmd: CreateTeamReport){
    println!("creating the new_team_report: {:?}", team_report_cmd);
    use crate::schema::team_report::dsl::*;

    let mut connection = establish_connection();
    let new_team_report = NewTeamReport {
        teams: &team_report_cmd.teams,
        sprint_num: team_report_cmd.sprint_num,
        understand_easiest: &team_report_cmd.understand_easiest,
        understand_hardest: &team_report_cmd.understand_hardest,
        approach_easiest: &team_report_cmd.approach_easiest,
        approach_hardest: &team_report_cmd.approach_hardest,
        solve_easiest: &team_report_cmd.solve_easiest,
        solve_hardest: &team_report_cmd.solve_hardest,
        evaluate_easiest: &team_report_cmd.evaluate_easiest,
        evaluate_hardest: &team_report_cmd.evaluate_hardest,
        completion: team_report_cmd.completion,
        contact: &team_report_cmd.contact,
        comments: &team_report_cmd.comments,
    };
    // DATABASE TARGET
    diesel::insert_into(team_report)
        .values(&new_team_report)
        .execute(&mut connection)
        .expect("Error saving new team report");
}

pub fn update_team_report(team_report_cmd: UpdateTeamReport){
    println!("updating team report: {:?}", team_report_cmd);
    use crate::schema::team_report::dsl::*;

    let mut connection = establish_connection();
    let new_team_report = UpdateTeamReport {
        sprint_num: team_report_cmd.sprint_num,
        understand_easiest: team_report_cmd.understand_easiest,
        understand_hardest: team_report_cmd.understand_hardest,
        approach_easiest: team_report_cmd.approach_easiest,
        approach_hardest: team_report_cmd.approach_hardest,
        solve_easiest: team_report_cmd.solve_easiest,
        solve_hardest: team_report_cmd.solve_hardest,
        evaluate_easiest: team_report_cmd.evaluate_easiest,
        evaluate_hardest: team_report_cmd.evaluate_hardest,
        completion: team_report_cmd.completion,
        contact: team_report_cmd.contact,
        comments: team_report_cmd.comments,
    };
    // DATABASE TARGET
}

pub fn show_team_report(){

}


