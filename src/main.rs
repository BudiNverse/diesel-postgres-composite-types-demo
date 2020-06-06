#[macro_use]
extern crate diesel;

mod schema;

use diesel::deserialize::FromSql;
use diesel::pg::types::sql_types::Record;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::{Varchar, Text};
use diesel::{
    deserialize, insert_into, serialize, serialize::WriteTuple, Connection, PgConnection,
    RunQueryDsl,
};
use diesel::{AsExpression, FromSqlRow, Identifiable, Insertable, Queryable};
use schema::persons;
use std::env;
use std::io::Write;
use dotenv::dotenv;

#[derive(Debug, Clone, Queryable, Identifiable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub pets: Vec<Pet>,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "persons"]
pub struct NewPerson {
    pub name: String,
    pub age: i32,
    pub pets: Vec<Pet>,
}

impl NewPerson {
    pub fn new(name: &str, age: i32, pets: Vec<Pet>) -> Self {
        NewPerson {
            name: name.into(),
            age,
            pets,
        }
    }
}

#[derive(SqlType)]
#[postgres(type_name = "animal_type")]
pub struct PgAnimalType;

#[derive(Debug, Clone, FromSqlRow, AsExpression, PartialEq)]
#[sql_type = "PgAnimalType"]
pub enum AnimalType {
    Cat,
    Fish,
}

impl From<&str> for AnimalType {
    fn from(s: &str) -> Self {
        match s {
            "Cat" => AnimalType::Cat,
            "Fish" => AnimalType::Fish,
            _ => AnimalType::Cat,
        }
    }
}

#[derive(Debug, Clone, FromSqlRow, AsExpression, PartialEq)]
#[sql_type = "PgPet"]
pub struct Pet {
    pub name: String,
    pub animal_type: AnimalType,
}

#[derive(SqlType)]
#[postgres(type_name = "pet")]
pub struct PgPet;


impl ToSql<PgAnimalType, Pg> for AnimalType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let animal_type = format!("{:?}", self);
        ToSql::<Text, Pg>::to_sql(&animal_type, out)
    }
}

impl FromSql<PgAnimalType, Pg> for AnimalType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        FromSql::<Text, Pg>::from_sql(bytes).map(|v: String| AnimalType::from(v.as_str()))
    }
}

impl ToSql<PgPet, Pg> for Pet {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Text, PgAnimalType)>::write_tuple(
            &(self.name.clone(), self.animal_type.clone()),
            out,
        )
    }
}

impl FromSql<PgPet, Pg> for Pet {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (name, animal_type) = FromSql::<Record<(Text, PgAnimalType)>, Pg>::from_sql(bytes)?;
        Ok(Pet { name, animal_type })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use schema::persons::dsl::*;

    let conn = establish_connection();

    let inverse = NewPerson::new(
        "iNverse",
        21,
        vec![
            Pet {
                name: "John".into(),
                animal_type: AnimalType::Cat,
            },
            Pet {
                name: "Cena".into(),
                animal_type: AnimalType::Fish,
            },
        ],
    );

    let res = insert_into(persons)
        .values(&inverse)
        .get_result::<Person>(&conn)?;

    dbg!(res);
    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
