use std::env;
pub mod config;
pub mod schema;
pub mod connection;
pub mod models;

pub fn say_hello() -> Result<(), std::io::Error>{
    let _x = dotenvy::dotenv();
    let db_url = env::var("DATABASE_URL");
    println!("{}", "Hi from db lib crate");
    println!("{:?} he", db_url);
    Ok(())
}