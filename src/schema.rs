// @generated automatically by Diesel CLI.

pub mod data {
    diesel::table! {
        data.fridge (id) {
            id -> Int4,
            #[max_length = 255]
            name -> Varchar,
        }
    }

    diesel::table! {
        data.fridge_item_link (item_id, fridge_id) {
            item_id -> Int4,
            fridge_id -> Int4,
        }
    }

    diesel::table! {
        data.home (id) {
            id -> Int4,
            #[max_length = 255]
            name -> Varchar,
        }
    }

    diesel::table! {
        data.home_fridge_link (home_id, fridge_id) {
            home_id -> Int4,
            fridge_id -> Int4,
        }
    }

    diesel::table! {
        data.item (id) {
            id -> Int4,
            #[max_length = 255]
            name -> Varchar,
            item_type_id -> Int4,
            expiration_date -> Date,
        }
    }

    diesel::table! {
        data.item_home_link (item_id, home_id) {
            item_id -> Int4,
            home_id -> Int4,
        }
    }

    diesel::table! {
        data.item_type (id) {
            id -> Int4,
            #[max_length = 255]
            name -> Varchar,
        }
    }

    diesel::table! {
        data.item_type_home_link (item_type_id, home_id) {
            item_type_id -> Int4,
            home_id -> Int4,
        }
    }

    diesel::table! {
        data.user (id) {
            id -> Int4,
            #[max_length = 255]
            email -> Varchar,
            #[max_length = 255]
            name -> Varchar,
        }
    }

    diesel::table! {
        data.user_home_link (user_id, home_id) {
            user_id -> Int4,
            home_id -> Int4,
            join_date -> Date,
            is_owner -> Bool,
        }
    }

    diesel::joinable!(fridge_item_link -> fridge (fridge_id));
    diesel::joinable!(fridge_item_link -> item (item_id));
    diesel::joinable!(home_fridge_link -> home (home_id));
    diesel::joinable!(item -> item_type (item_type_id));
    diesel::joinable!(item_home_link -> home (home_id));
    diesel::joinable!(item_home_link -> item (item_id));
    diesel::joinable!(item_type_home_link -> home (home_id));
    diesel::joinable!(item_type_home_link -> item_type (item_type_id));
    diesel::joinable!(user_home_link -> home (home_id));
    diesel::joinable!(user_home_link -> user (user_id));

    diesel::allow_tables_to_appear_in_same_query!(
        fridge,
        fridge_item_link,
        home,
        home_fridge_link,
        item,
        item_home_link,
        item_type,
        item_type_home_link,
        user,
        user_home_link,
    );
}
