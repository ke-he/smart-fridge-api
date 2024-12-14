// @generated automatically by Diesel CLI.

diesel::table! {
    fridge (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_by -> Int4,
        created_at -> Date,
    }
}

diesel::table! {
    home (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_by -> Int4,
        created_at -> Date,
    }
}

diesel::table! {
    home_fridge_link (home_id, fridge_id) {
        home_id -> Int4,
        fridge_id -> Int4,
    }
}

diesel::table! {
    item (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        item_type_id -> Int4,
        #[max_length = 255]
        code -> Nullable<Varchar>,
        created_by -> Int4,
        created_at -> Date,
    }
}

diesel::table! {
    item_fridge_link (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        fridge_id -> Nullable<Int4>,
        expiration_date -> Date,
    }
}

diesel::table! {
    item_type (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_by -> Int4,
        created_at -> Date,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    user_home_link (user_id, home_id) {
        user_id -> Int4,
        home_id -> Int4,
        join_date -> Date,
    }
}
diesel::table! {
    item_type_home_link (item_type_id, home_id) {
        item_type_id -> Int4,
        home_id -> Int4,
    }
}

diesel::joinable!(fridge -> user (created_by));
diesel::joinable!(home -> user (created_by));
diesel::joinable!(home_fridge_link -> fridge (fridge_id));
diesel::joinable!(home_fridge_link -> home (home_id));
diesel::joinable!(item -> item_type (item_type_id));
diesel::joinable!(item -> user (created_by));
diesel::joinable!(item_fridge_link -> fridge (fridge_id));
diesel::joinable!(item_fridge_link -> item (item_id));
diesel::joinable!(item_type -> user (created_by));
diesel::joinable!(user_home_link -> home (home_id));
diesel::joinable!(user_home_link -> user (user_id));
diesel::joinable!(item_type_home_link -> item_type (item_type_id));
diesel::joinable!(item_type_home_link -> home (home_id));

diesel::allow_tables_to_appear_in_same_query!(
    fridge,
    home,
    home_fridge_link,
    item,
    item_fridge_link,
    item_type,
    user,
    user_home_link,
    item_type_home_link,
);
