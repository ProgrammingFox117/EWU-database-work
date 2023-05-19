use crate::args::{SprintSubcommand, SprintCommand,
                  CreateSprint, UpdateSprint};
use crate::db::establish_connection;
use crate::models::{NewSprint, SprintNumDateReturn};
use diesel::prelude::*;


pub fn handle_sprint_command(sprintcmd: SprintCommand){
    let command = sprintcmd.command;
    match command{
        SprintSubcommand::Create(sprintcmd) =>{
            create_sprint(sprintcmd);
        }
        SprintSubcommand::Update(sprintcmd) =>{
            update_sprint(sprintcmd);
        }
        SprintSubcommand::Show => {
            show();
        }
    }
}
pub fn create_sprint(sprintcmd: CreateSprint){
    println!("creating the sprint: {:?}", sprintcmd);
    use crate::schema::sprintnum_date::dsl::*;

    let mut connection = establish_connection();
    let new_sprint = NewSprint {
        sprint_num: sprintcmd.sprint_num,
        sprint_date: &sprintcmd.sprint_date,
    };
                    // DATABASE TARGET
    diesel::insert_into(sprintnum_date)
        .values(&new_sprint)
        .execute(&mut connection)
        .expect("Error saving new sprint");
}
// not needed...
pub fn update_sprint(sprintcmd: UpdateSprint) {
    println!("updating the sprint: {:?}", sprintcmd);
    use crate::schema::sprintnum_date::dsl::*;

    let mut connection = establish_connection();
    let target = sprintnum_date.filter(sprint_num.eq(&sprintcmd.sprint_num));
    let updated_row = diesel::update(target)
        .set(sprint_date.eq(&sprintcmd.sprint_date))
        .execute(&mut connection)
        .expect("Error updating sprint");

    println!("Updated {} rows", updated_row);

}
pub fn show(){

}

