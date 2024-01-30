// @generated automatically by Diesel CLI.

diesel::table! {
    employees (identifier) {
        #[max_length = 3]
        identifier -> Varchar,
        #[max_length = 2]
        department -> Varchar,
        #[max_length = 2]
        role -> Varchar,
        #[max_length = 25]
        first_name -> Varchar,
        #[max_length = 25]
        last_name -> Varchar,
    }
}
