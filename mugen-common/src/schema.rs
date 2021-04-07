table! {
    documents (id) {
        id -> Int4,
        created -> Timestamptz,
        last_updated -> Nullable<Timestamptz>,
        filetype -> Nullable<Varchar>,
        version -> Int4,
        size -> Int8,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hash -> Text,
        email -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    documents,
    users,
);
