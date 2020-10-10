table! {
    use diesel::sql_types::*;
    use crate::enums::product_kind::ProductKind;

    products (id) {
        id -> Int4,
        storage_id -> Int4,
        name -> Text,
        amount -> Int4,
        peremption_date -> Date,
        product_kind -> ProductKind,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::enums::storage_kind::StorageKind;

    storages (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        storage_kind -> StorageKind,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;
    use crate::enums::*;

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
