// @generated automatically by Diesel CLI.

diesel::table! {
    fumos (id) {
        id -> Int8,
        caption -> Varchar,
        img -> Varchar,
        public -> Bool,
        involved -> Nullable<Array<Nullable<Text>>>,
        attribution -> Nullable<Text>,
        submission_time -> Timestamp,
        #[max_length = 32]
        submitter -> Varchar,
    }
}
