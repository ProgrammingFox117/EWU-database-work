use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct sprintnum_date_return {
    pub sprint_num: i8,
    pub sprint_date: Option<String>,
}

#[derive(Insertable)]
///            "this is the the database target!!!"
#[diesel(table_name = sprintnum_date)]
pub struct NewSprint<'a> {
    pub sprint_num: &'a i8,
    pub sprint_date: &'a str,
}

#[derive(Queryable)]
pub struct Team_report {
    pub sprint: i8,
    pub understand_easiest: Option<String>,
    pub understand_hardest: Option<String>,
    pub approach_easiest: Option<String>,
    pub approach_hardest: Option<String>,
    pub solve_easiest: Option<String>,
    pub solve_hardest: Option<String>,
    pub evaluate_easiest: Option<String>,
    pub evaluate_hardest: Option<String>,
    pub completion: f32,
    pub contact: Option<String>,
    pub comments: Option<String>,
}
#[derive(Insertable)]
#[diesel(table_name = team_report)]
pub struct NewTeamReport<'a> {
    pub sprint: &'a i8,
    pub sprint_data: &'a str,
    pub understand_easiest: &'a str,
    pub understand_hardest: &'a str,
    pub approach_easiest: &'a str,
    pub approach_hardest: &'a str,
    pub solve_easiest: &'a str,
    pub solve_hardest: &'a str,
    pub evaluate_easiest: &'a str,
    pub evaluate_hardest: &'a str,
    pub completion: &'a f32,
    pub contact: &'a str,
    pub comments: &'a str,
}

#[derive(Queryable)]
pub struct Individual_report {
    pub monday_time:f32,
    pub tuesday_time:f32,
    pub wednesday_time:f32,
    pub thursday_time:f32,
    pub friday_time:f32,
    pub saturday_time:f32,
    pub sunday_time:f32,
    pub discrepancy: Option<String>,
    pub request: Option<String>,
    pub ouath_id: f32
}

#[derive(Insertable)]
#[diesel(table_name = individual_report)]
pub struct NewIndividualReport<'a> {
    pub monday_time: &'a f32,
    pub tuesday_time: &'a f32,
    pub wednesday_time: &'a f32,
    pub thursday_time: &'a f32,
    pub friday_time: &'a f32,
    pub saturday_time: &'a f32,
    pub sunday_time: &'a f32,
    pub discrepancy: &'a str,
    pub request: &'a str,
    pub ouath_id: &'a f32
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
    pub sprint: Option<String>,
    pub activity_index: Option<String>,
    pub answer: Option<String>,
    pub ouath_id: Option<String>
}