// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        #[max_length = 50]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_details (user_id) {
        #[max_length = 15]
        user_id -> Varchar,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        avatar_url -> Nullable<Text>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 15]
        id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Nullable<Varchar>,
        #[max_length = 50]
        role_id -> Varchar,
        #[max_length = 255]
        google_id -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(user_details -> users (user_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(roles, user_details, users,);
