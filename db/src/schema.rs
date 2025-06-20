// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        user_name -> Varchar,
        password -> Varchar,
        name -> Nullable<Text>,
    }
}
