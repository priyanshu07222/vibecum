use std::env;

pub struct Config{
    pub db_url: String
}

impl Config {
    pub fn new() -> Config {
        let url = env::var("DATABASE_URL");
        match url {
            Ok(x) => {
                let db_url = x;
                Config {
                    db_url
                }
            }
            Err(_) => {
                panic!("Please provide the database_url enviroment varibale")
            }
        }
    }
}