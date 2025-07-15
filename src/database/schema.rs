
diesel::table! {
    fumos (id) {
        id -> Int4,
        caption -> Text,
        img -> VarChar,
        public -> Bool,
        involved -> Array<Text>,
        attribution -> Text
    }
}