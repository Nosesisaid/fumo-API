// @generated automatically by Diesel CLI.

diesel::table! {
    fumos (id) {
        id -> Int8,
        caption -> Varchar,
        img -> Varchar,
        public -> Bool,
        involved -> Array<Nullable<Text>>,
        attribution -> Text,
        submission_time -> Timestamp,
        submitter -> Text,
    }
}
