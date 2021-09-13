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
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(documents, users);
