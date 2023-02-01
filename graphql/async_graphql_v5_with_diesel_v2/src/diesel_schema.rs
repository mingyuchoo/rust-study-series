// @generated automatically by Diesel CLI.

diesel::table! {
    members (id) {
        id -> Int4,
        name -> Varchar,
        knockouts -> Int4,
        team_id -> Int4,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(members -> teams (team_id));

diesel::allow_tables_to_appear_in_same_query!(
    members,
    teams,
);
