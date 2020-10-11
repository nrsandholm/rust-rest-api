table! {
    applicants (id) {
        id -> Int4,
        first_name -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
    }
}

table! {
    applications (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    applications_applicants (id) {
        id -> Int4,
        application_id -> Int4,
        applicant_id -> Int4,
    }
}

table! {
    files (id) {
        id -> Int4,
        application_id -> Int4,
        uri -> Varchar,
    }
}

joinable!(applications_applicants -> applicants (applicant_id));
joinable!(applications_applicants -> applications (application_id));
joinable!(files -> applications (application_id));

allow_tables_to_appear_in_same_query!(
    applicants,
    applications,
    applications_applicants,
    files,
);
