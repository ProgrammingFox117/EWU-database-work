use crate::args::{
    SprintSubcommand,
    SprintCommand,
    CreateSprint,
    UpdateUser,
    DeleteEntity,
};
use crate::db::establish_connection;
use crate::models::{NewSprint, sprintnum_date_return};
use diesel::prelude::*;

pub fn handle_sprint_command(sprintcmd: SprintCommand){
    let command = sprintcmd.command;
    match command{
        SprintSubcommand::Create(sprintcmd) =>{
            create_sprint(sprintcmd);
        }
    }
}
pub fn create_sprint(sprintcmd: CreateSprint){
    println!("creating the sprint: {:?}", sprintcmd);
    use crate::schema::sprintnum_date::dsl::*;

    let connection = establish_connection();
    let new_sprint = NewSprint {
        sprint_num: &sprintcmd.SprintNum,
        sprint_date: &sprintcmd.sprint_date,
    };
                    /// DATABASE TARGET
    diesel::insert_into(sprintnum_date)
        .values(&new_sprint)
        .execute(&connection)
        .expect("Error saving new sprint");
}

