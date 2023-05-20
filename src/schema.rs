// @generated automatically by Diesel CLI.

diesel::table! {
    individual_report (ouath_id, sprint_num) {
        ouath_id -> Varchar,
        sprint_num -> Integer,
        monday_time -> Nullable<Integer>,
        tuesday_time -> Nullable<Integer>,
        wednesday_time -> Nullable<Integer>,
        thursday_time -> Nullable<Integer>,
        friday_time -> Nullable<Integer>,
        saturday_time -> Nullable<Integer>,
        sunday_time -> Nullable<Integer>,
        discrepancy -> Nullable<Varchar>,
        request -> Nullable<Varchar>,
    }
}

diesel::table! {
    sprintnum_date (sprint_num) {
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
    individual_report,
    sprintnum_date,
    team_report,
);
