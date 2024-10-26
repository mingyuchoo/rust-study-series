// @generated automatically by Diesel CLI.

diesel::table! {
    author (id) {
        id -> Int4,
        name -> Varchar,
        country -> Varchar,
    }
}

diesel::table! {
    book (id) {
        id -> Int4,
        title -> Varchar,
        author_id -> Int4,
    }
}

diesel::table! {
    customer (id) {
        id -> Int4,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        username -> Nullable<Text>,
        ip_address -> Nullable<Text>,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        name -> Varchar,
        knockouts -> Int4,
        team_id -> Int4,
    }
}

diesel::table! {
    order (id) {
        id -> Int4,
        transaction_id -> Nullable<Text>,
        product -> Nullable<Text>,
        purchase_price -> Nullable<Text>,
        discount_price -> Nullable<Text>,
        order_date -> Nullable<Text>,
        customer_id -> Nullable<Int4>,
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

diesel::table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(book -> author (author_id));
diesel::joinable!(members -> teams (team_id));
diesel::joinable!(order -> customer (customer_id));

diesel::allow_tables_to_appear_in_same_query!(author, book, customer, members, order, posts, teams,);
