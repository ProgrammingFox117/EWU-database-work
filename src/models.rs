use chrono::{Datelike, NaiveDate};
use crate::schema::sprintnum_date;
use crate::schema::team_report;
use diesel::prelude::*;





#[derive(Queryable)]
pub struct SprintNumDateReturn {
    pub sprint_num: i32,
    pub sprint_date: NaiveDate,
}

#[derive(Insertable)]
//            "this is the the database target!!!"
#[diesel(table_name = sprintnum_date)]
pub struct NewSprint<'a> {
    pub sprint_num: i32,
    pub sprint_date: &'a NaiveDate,
}
#[derive(Queryable)]
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
#[diesel(table_name = team_report)]
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

