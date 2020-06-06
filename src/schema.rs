table! {
    use diesel::sql_types::*;
    use crate::PgPet;
    persons (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
        pets -> Array<PgPet>,
    }
}
