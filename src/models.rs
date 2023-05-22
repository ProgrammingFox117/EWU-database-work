use crate::schema::individual_reports;
use crate::schema::sprint_num_dates;
use crate::schema::team_reports;
use chrono::NaiveDate;

#[derive(Queryable, Debug, AsChangeset)]
pub struct SprintNumDate {
    pub sprint_num: i32,
    pub sprint_date: NaiveDate,
}

#[derive(Insertable)]
// db target must be specified as table_name
#[diesel(table_name = sprint_num_dates)]
pub struct NewSprint<'a> {
    pub sprint_num: i32,
    pub sprint_date: &'a NaiveDate,
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct TeamReport {
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
    pub completion: f32,
    pub contact: String,
    pub comments: String,
}

#[derive(Insertable)]
#[diesel(table_name = team_reports)]
pub struct NewTeamReport<'a> {
    pub teams: &'a str,
    pub sprint_num: i32,
    pub understand_easiest: &'a str,
    pub understand_hardest: &'a str,
    pub approach_easiest: &'a str,
    pub approach_hardest: &'a str,
    pub solve_easiest: &'a str,
    pub solve_hardest: &'a str,
    pub evaluate_easiest: &'a str,
    pub evaluate_hardest: &'a str,
    pub completion: f32,
    pub contact: &'a str,
    pub comments: &'a str,
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct IndividualReport {
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

#[derive(Insertable)]
#[diesel(table_name = individual_reports)]
pub struct NewIndividualReport<'a> {
    pub ouath_id: &'a str,
    pub sprint_num: i32,
}

/*#[derive(Queryable)]
pub struct Requirements
{
    pub class       : String,
    pub teams       : String,
    pub indexs      : i32,
    pub description : String,
}

#[derive(Insertable)]
#[diesel(table_name = requirements)]
pub struct NewRequirements<'a> {

    pub class : &'a str,
    pub teams  : &'a str,
    pub indexs  : i32,
    pub description  : &'a str,

}*/

/*#[derive(Queryable)]
pub struct TeamActivities
{
    pub teams           : String,
    pub ouath_id        : String,
    pub sprint_num      : i32,
    pub activity_index  : i32,
    pub answers         : String,
}

#[derive(Insertable)]
#[diesel(table_name = team_activities)]
pub struct NewTeamActivities<'a> {

    pub teams  : &'a str,
    pub teams           : &'a str,
    pub ouath_id        : &'a str,
    pub sprint_num      : i32,
    pub activity_index  : i32,
    pub answers         : &'a str,

}*/
