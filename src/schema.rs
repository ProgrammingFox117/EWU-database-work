// @generated automatically by Diesel CLI.

diesel::table! {
    sprintnum_date (sprint_num, sprint_date) {
        sprint_num -> Integer,
        sprint_date -> Date,
    }
}

diesel::table! {
    team_report (teams, sprint_num) {
        teams -> Varchar,
        sprint_num -> Integer,
        understand_easiest -> Nullable<Varchar>,
        understand_hardest -> Nullable<Varchar>,
        approach_easiest -> Nullable<Varchar>,
        approach_hardest -> Nullable<Varchar>,
        solve_easiest -> Nullable<Varchar>,
        solve_hardest -> Nullable<Varchar>,
        evaluate_easiest -> Nullable<Varchar>,
        evaluate_hardest -> Nullable<Varchar>,
        completion -> Nullable<Float>,
        contact -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sprintnum_date,
    team_report,
);
