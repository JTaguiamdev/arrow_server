// @generated automatically by Diesel CLI.

diesel::table! {
    user_roles (role_id) {
        role_id -> Integer,
        user_id -> Integer,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(user_roles, users,);
