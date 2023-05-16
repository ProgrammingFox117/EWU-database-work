use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct DatabaseArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    ///Create, update, delete, show users
    User(UserCommand),
    ///Create, update, delete
    Sprint(SprintCommand),
}




///start User command
#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),

    /// Update an existing user
    Update(UpdateUser),

    /// Delete a user
    Delete(DeleteEntity),

    /// Show all users
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,

}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The id of the user to update
    pub id: i32,

    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,

}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}


#[derive(Debug, Args)]
pub struct SprintCommand {
    #[clap(subcommand)]
    pub command: SprintSubcommand,
}

///start Sprint commands
#[derive(Debug, Subcommand)]
pub enum SprintSubcommand {
    /// Create a new sprint
    Create(CreateSprint),

    /// Update an existing sprint
    Update(UpdateSprint),

    /// Delete a the sprint
    Delete(DeleteEntity),

    /// Show all Sprints
    Show,
}

#[derive(Debug, Args)]
pub struct CreateSprint {
    /// The title of the video to create
    pub SprintNum: i8,

    /// The description of the video to create
    pub sprint_date: String,
}

#[derive(Debug, Args)]
pub struct UpdateSprint {
    /// The id of the video to update
    pub sprintNum: i8,

    /// The title of the video
    pub sprint_date: String,

}