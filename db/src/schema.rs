// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    message (id) {
        #[max_length = 100]
        id -> Varchar,
        text -> Text,
        created_at -> Nullable<Timestamp>,
        #[max_length = 100]
        user_id -> Varchar,
        #[max_length = 100]
        room_id -> Varchar,
    }
}

diesel::table! {
    room (id) {
        #[max_length = 100]
        id -> Varchar,
        #[max_length = 9]
        code -> Bpchar,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        admin_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    room_user (id) {
        #[max_length = 100]
        id -> Varchar,
        #[max_length = 100]
        user_id -> Varchar,
        #[max_length = 100]
        room_id -> Varchar,
        role_assign -> Nullable<Role>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        #[max_length = 150]
        user_name -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        name -> Nullable<Text>,
        create_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(message -> room (room_id));
diesel::joinable!(message -> users (user_id));
diesel::joinable!(room -> users (admin_id));
diesel::joinable!(room_user -> room (room_id));
diesel::joinable!(room_user -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    message,
    room,
    room_user,
    users,
);
