use diesel::{Connection, PgConnection};

use crate::config::{Config};

pub struct Connect{
    pub conn: PgConnection
}

impl Default for Connect{
    fn default() -> Self {
        let config = Config::new();
        let conn = PgConnection::establish(&config.db_url).unwrap();
        Connect { conn }
    }
}