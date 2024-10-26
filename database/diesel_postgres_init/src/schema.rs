// @generated automatically by Diesel CLI.

diesel::table! {
    member (id) {
        id -> Int4,
        name -> Varchar,
        role -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(member, posts,);
