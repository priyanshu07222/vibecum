use std::env;



pub struct config{
    db_url: String
}

impl config {
    pub fn new(&self) -> config {
        let url = env::var("DATABASE_URL");
        match url {
            Ok(x) => {
                let db_url = x;
                Self {
                    db_url
                }
            }
            Err(_) => {
                panic!("Please provide the database_url enviroment varibale")
            }
        }
    }
}