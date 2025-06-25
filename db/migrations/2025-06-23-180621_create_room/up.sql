-- Your SQL goes here
CREATE TABLE room (
 id            VARCHAR(100) NOT NULL UNIQUE PRIMARY,
 code          CHAR(9) UNIQUE NOT NULL,
 name          VARCHAR(50),
 admin_id      VARCHAR(100)
 created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
 CONSTRAINT admin FOREIGN KEY (adminId) REFERENCES users(id)
);


CREATE TABLE room_user(
id          VARCHAR(100) NOT NULL UNIQUE PRIMARY,
user_id     VARCHAR(100) NOT NULL,
room_id     VARCHAR(100) NOT NULL,
role_assign role DEFAULT "USER"
CONSTRAINT user FOREIGN KEY (user_id) REFERENCES users(id),
CONSTRAINT user FOREIGN KEY (room_id) REFERENCES room(id)
);

CREATE TABLE message(
id             VARCHAR(100) PRIMARY,

)


CREATE TYPE role AS ENUM ("USER" , "ADMIN");