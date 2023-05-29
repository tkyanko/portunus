// @generated automatically by Diesel CLI.

diesel::table! {
    keys (id) {
        id -> Integer,
        product_id -> Integer,
        product_key -> Text,
        time_added -> Timestamp,
        user_who_added -> Integer,
        time_claimed -> Nullable<Timestamp>,
        user_who_claimed -> Nullable<Integer>,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        title -> Text,
        store -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        discord_id -> Text,
        last_taken_time -> Timestamp,
        keys_given -> Integer,
        keys_taken -> Integer,
    }
}

diesel::joinable!(keys -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    keys,
    products,
    users,
);
