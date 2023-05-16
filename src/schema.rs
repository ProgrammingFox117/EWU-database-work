// @generated automatically by Diesel CLI.

diesel::table! {
    contact (ouath_id) {
        ouath_id -> Varchar,
        class -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
    }
}

diesel::table! {
    individual_report (ouath_id) {
        ouath_id -> Varchar,
        sprint_num -> Nullable<Integer>,
        monday_time -> Nullable<Float>,
        tuesday_time -> Nullable<Float>,
        wednesday_time -> Nullable<Float>,
        thursday_time -> Nullable<Float>,
        friday_time -> Nullable<Float>,
        saturday_time -> Nullable<Float>,
        sunday_time -> Nullable<Float>,
        discrepancy -> Nullable<Varchar>,
        request -> Nullable<Varchar>,
    }
}

diesel::table! {
    login (ouath_id) {
        ouath_id -> Varchar,
        is_teacher -> Nullable<Bool>,
        is_student -> Nullable<Bool>,
        teams -> Nullable<Varchar>,
        class -> Nullable<Varchar>,
        is_Admin -> Nullable<Bool>,
    }
}

diesel::table! {
    requirements (indexs, teams) {
        class -> Nullable<Varchar>,
        teams -> Varchar,
        indexs -> Integer,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    sprintnum_date (sprint_num, sprint_date) {
        sprint_num -> Integer,
        sprint_date -> Date,
    }
}

diesel::table! {
    team_activities (ouath_id) {
        teams -> Nullable<Varchar>,
        ouath_id -> Varchar,
        sprint_num -> Nullable<Integer>,
        activity_index -> Nullable<Integer>,
        answers -> Nullable<Varchar>,
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
        completion -> Nullable<Integer>,
        contact -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
    }
}

diesel::joinable!(contact -> login (ouath_id));
diesel::joinable!(login -> individual_report (ouath_id));
diesel::joinable!(login -> team_activities (ouath_id));

diesel::allow_tables_to_appear_in_same_query!(
    contact,
    individual_report,
    login,
    requirements,
    sprintnum_date,
    team_activities,
    team_report,
);
