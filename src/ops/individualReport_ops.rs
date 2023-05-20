use crate::args::{individualReportSubcommand, individualReportCommand, CreateindividualReport,
                  UpdateUser, DeleteEntity, UpdateindividualReport};
use crate::db::establish_connection;
use crate::models::{NewindividualReport, Individual_report};
use diesel::prelude::*;
use diesel::sql_types::*;

pub fn handle_individualReport_command(individualReportcmd: individualReportCommand){
    let command = individualReportcmd.command;
    match command{
        individualReportSubcommand::Create(individualReportcmd) =>{
            create_individualReport(individualReportcmd);
        }
        individualReportSubcommand::Update(individualReportcmd) =>{
            update_individualReport(individualReportcmd);
        }
        individualReportSubcommand::Show => {
            show();
        }
    }
}
pub fn create_individualReport(individualReportcmd: CreateindividualReport){
    println!("creating the individualReport: {:?}", individualReportcmd);
    use crate::schema::individualReportnum_date::dsl::*;

    let mut connection = establish_connection();
    let new_individualReport = NewindividualReport {
        individualReport_monday_time: &individualReportcmd.monday_time,
        individualReport_tuesday_time: &individualReportcmd.tuesday_time,
        individualReport_wednesday_time: &individualReportcmd.wednesday_time,
        individualReport_thursday_time: &individualReportcmd.thursday_time,
        individualReport_friday_time: &individualReportcmd.friday_time,
        individualReport_saturday_time: &individualReportcmd.saturday_time,
        individualReport_sunday_time: &individualReportcmd.sunday_time,

        individualReport_discrepancy: &individualReportcmd.discrepancy,
        individualReport_request: &individualReportcmd.request,
        individualReport_ouath_id: &individualReportcmd.ouath_id,

    };
    // DATABASE TARGET
    diesel::insert_into(individualReportnum_date)
        .values(&new_individualReport)
        .execute(&mut connection)
        .expect("Error saving new individualReport");
}
pub fn update_individualReport(individualReportcmd: UpdateindividualReport) {
    println!("updating the individualReport: {:?}", individualReportcmd);
    use crate::schema::Individual_report::dsl::*;

    let mut connection = establish_connection();
    let target = Individual_report.filter(individualReport_num.eq(&individualReportcmd.Individual_report));
    let updated_row = diesel::update(target)
        .set(Individual_report.eq(&individualReportcmd.Individual_report))
        .execute(&mut connection)
        .expect("Error updating individualReport");

    println!("Updated {} rows", updated_row);

}
pub fn show(){
    println!("Induival reports");
    use crate::chema::videos::dsl::*;

    let connection = establish_connection();

    let results = videos
        .filter(removed.eq(false))
        .load::<Individual_report>(conn: &connection)
        .expect("error loading Individual_report");
}