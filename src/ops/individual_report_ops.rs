use crate::args::{IndividualReportSubcommand, IndividualReportCommand, CreateIndividualReport,
                  UpdateIndividualReport};
use crate::db::establish_connection;
use crate::models::{NewIndividualReport, IndividualReport};
use diesel::prelude::*;
use diesel::sql_types::*;

pub fn handle_individual_report_command(individual_report_cmd: IndividualReportCommand){
    let command = individual_report_cmd.command;
    match command{
        IndividualReportSubcommand::Create(individual_report_cmd) =>{
            create_individual_report(individual_report_cmd);
        }
        IndividualReportSubcommand::Update(individual_report_cmd) =>{
            update_individual_report(individual_report_cmd);
        }
        IndividualReportSubcommand::Show => {
            show();
        }
    }
}
pub fn create_individual_report(individual_report_cmd: CreateIndividualReport){
    println!("creating the Individual Report: {:?}", individual_report_cmd);
    use crate::schema::individual_report::dsl::*;

    let mut connection = establish_connection();
    let new_individual_report = NewIndividualReport {
        ouath_id: &individual_report_cmd.ouath_id,
        sprint_num: individual_report_cmd.sprint_num,
    };
    // DATABASE TARGET
    diesel::insert_into(individual_report)
        .values(&new_individual_report)
        .execute(&mut connection)
        .expect("Error saving new individualReport");
}
pub fn update_individual_report(individual_report_cmd: UpdateIndividualReport) {
    /*println!("updating the individualReport: {:?}", individual_report_cmd);
    use crate::schema::individual_report::dsl::*;

    let mut connection = establish_connection();
    let target = IndividualReport.filter(IndividualReport.eq(&individual_report_cmd.individual_report));
    let updated_row = diesel::update(target)
        .set(individual_report.eq(&individual_report_cmd.individual_report))
        .execute(&mut connection)
        .expect("Error updating individualReport");

    println!("Updated {} rows", updated_row);*/

}
pub fn show(){

}