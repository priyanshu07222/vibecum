-- Your SQL goes here
CREATE table users (
    id          TEXT NOT NULL PRIMARY KEY,
    user_name   VARCHAR(50) NOT NULL,
    password    VARCHAR(50) NOT NULL,  
    name        TEXT,
    create_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)