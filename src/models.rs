use chrono::{Datelike, NaiveDate};
use crate::schema::*;
use diesel::prelude::*;
use diesel::sql_types::*;




#[derive(Queryable)]
pub struct sprintnum_date_return {
    pub sprint_num: i32,
    pub sprint_date: NaiveDate,
}

#[derive(Insertable)]
///            "this is the the database target!!!"
#[diesel(table_name = sprintnum_date)]
pub struct NewSprint<'a> {
    pub sprint_num: i32,
    pub sprint_date: &'a NaiveDate,
}

#[derive(Queryable)]
pub struct Team_report {
    pub sprint_num: i32,
    pub understand_easiest: String,
    pub understand_hardest: String,
    pub approach_easiest: String,
    pub approach_hardest: String,
    pub solve_easiest: String,
    pub solve_hardest: String,
    pub evaluate_easiest: String,
    pub evaluate_hardest: String,
    pub completion: Float,
    pub contact: String,
    pub comments: String,
}
#[derive(Insertable)]
#[diesel(table_name = team_report)]
pub struct NewTeamReport<'a> {
    pub sprint_num: i32,
    pub understand_easiest: &'a str,
    pub understand_hardest: &'a str,
    pub approach_easiest: &'a str,
    pub approach_hardest: &'a str,
    pub solve_easiest: &'a str,
    pub solve_hardest: &'a str,
    pub evaluate_easiest: &'a str,
    pub evaluate_hardest: &'a str,
    pub completion: Float,
    pub contact: &'a str,
    pub comments: &'a str,
}

#[derive(Queryable)]
pub struct Individual_report {
    pub monday_time:Float,
    pub tuesday_time:Float,
    pub wednesday_time:Float,
    pub thursday_time:Float,
    pub friday_time:Float,
    pub saturday_time:Float,
    pub sunday_time: Float,
    pub discrepancy: String,
    pub request: String,
    pub ouath_id: Float
}

#[derive(Insertable)]
#[diesel(table_name = individual_report)]
pub struct NewIndividualReport<'a> {
    pub monday_time: Float,
    pub tuesday_time: Float,
    pub wednesday_time: Float,
    pub thursday_time: Float,
    pub friday_time: Float,
    pub saturday_time: Float,
    pub sunday_time: Float,
    pub discrepancy: Float,
    pub request: &'a str,
    pub ouath_id: Float
}

#[derive(Queryable)]
pub struct Login {
    pub is_teacher:  bool,
    pub is_student: Option<String>,
    pub admin: Option<String>,
    pub team: Option<String>,
    pub class: Option<String>,
    pub ouath_id: Option<String>
}

#[derive(Queryable)]
pub struct Requirements {
    pub class:  Option<String>,
    pub description: Option<String>,
    pub teams: Option<String>,
    pub indexs: Option<String>
}

#[derive(Queryable)]
pub struct Contact {
    pub class:  Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub ouath_id: Option<String>
}

#[derive(Queryable)]
pub struct Team_activities {
    pub teams:  Option<String>,
    pub sprint_num: Option<String>,
    pub activity_index: Option<String>,
    pub answer: Option<String>,
    pub ouath_id: Option<String>
}