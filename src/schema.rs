// @generated automatically by Diesel CLI.

diesel::table! {
    role_services (role_id, service_id) {
        role_id -> Uuid,
        service_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    services (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Nullable<Varchar>,
        role_id -> Uuid,
        #[max_length = 255]
        google_id -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(role_services -> roles (role_id));
diesel::joinable!(role_services -> services (service_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(role_services, roles, services, users,);
