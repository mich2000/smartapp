table! {
    use diesel::sql_types::*;

    products (id) {
        id -> Int4,
        storage_id -> Int4,
        name -> Text,
        amount -> Int2,
        peremption_date -> Date,
        product_kind -> crate::enums::product_kind::Product_Kind,
    }
}

table! {
    use diesel::sql_types::*;

    storages (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        storage_kind -> crate::enums::storage_kind::Storage_Kind,
    }
}

table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        salt -> Text,
    }
}

joinable!(products -> storages (storage_id));
joinable!(storages -> users (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    storages,
    users,
);
