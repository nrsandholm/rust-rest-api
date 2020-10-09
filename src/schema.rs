table! {
    applications (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    files (id) {
        id -> Int4,
        application_id -> Int4,
        uri -> Varchar,
    }
}

joinable!(files -> applications (application_id));

allow_tables_to_appear_in_same_query!(
    applications,
    files,
);
