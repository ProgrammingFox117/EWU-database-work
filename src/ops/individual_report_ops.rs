use crate::args::{
    CreateIndividualReport, IndividualReportCommand, IndividualReportSubcommand,
    UpdateIndividualReport,
};
use crate::db::establish_connection;
use crate::models::{IndividualReport, NewIndividualReport};
use diesel::prelude::*;

pub fn handle_individual_report_command(individual_report_cmd: IndividualReportCommand) {
    let command = individual_report_cmd.command;
    match command {
        IndividualReportSubcommand::Create(individual_report_cmd) => {
            create_individual_report(individual_report_cmd);
        }
        IndividualReportSubcommand::Update(individual_report_cmd) => {
            update_individual_report(individual_report_cmd);
        }
    }
}

pub fn create_individual_report(individual_report_cmd: CreateIndividualReport) {
    println!(
        "creating the Individual Report: {:?}",
        individual_report_cmd
    );
    use crate::schema::individual_reports::dsl::*;

    let connection = &mut establish_connection();
    let new_individual_report = NewIndividualReport {
        ouath_id: &individual_report_cmd.ouath_id,
        sprint_num: individual_report_cmd.sprint_num,
    };
    // DATABASE TARGET
    diesel::insert_into(individual_reports)
        .values(&new_individual_report)
        .execute(connection)
        .expect("Error saving new individualReport");
}

pub fn update_individual_report(individual_report_cmd: UpdateIndividualReport) {
    println!("updating the individualReport: {:?}", individual_report_cmd);
    use crate::schema::individual_reports::dsl::*;

    let connection = &mut establish_connection();
    let new_individual_report = IndividualReport {
        ouath_id: individual_report_cmd.ouath_id.clone(),
        sprint_num: individual_report_cmd.sprint_num,
        monday_time: individual_report_cmd.monday_time,
        tuesday_time: individual_report_cmd.tuesday_time,
        wednesday_time: individual_report_cmd.wednesday_time,
        thursday_time: individual_report_cmd.thursday_time,
        friday_time: individual_report_cmd.friday_time,
        saturday_time: individual_report_cmd.saturday_time,
        sunday_time: individual_report_cmd.sunday_time,
        discrepancy: individual_report_cmd.discrepancy,
        request: individual_report_cmd.request,
    };

    let updated_row = diesel::update(individual_reports.find((
        individual_report_cmd.ouath_id,
        individual_report_cmd.sprint_num,
    )))
    .set(&new_individual_report)
    .execute(connection)
    .expect("Error updating individualReport");

    println!("Updated {} rows", updated_row);
}
