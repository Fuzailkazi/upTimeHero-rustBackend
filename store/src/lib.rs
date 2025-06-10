use std::{env::{self,VarError}, fmt::Error};
use diesel::prelude::*;
use dotenvy::dotenv;

pub mod schema;

pub struct Store {
    pub conn: PgConnection,
}

impl Store{
    fn default()-> Result<Self, ConnectionError>{
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("please provide the database_url environment variable"))
        let conn = PgConnection::establish(&db_url)?;
        Ok(Self{
            conn
        })
    }
}

impl Store{
    pub fn create_user(&self){}

    pub fn crate_website(&self)-> String{
        String::from("1")
    }
}