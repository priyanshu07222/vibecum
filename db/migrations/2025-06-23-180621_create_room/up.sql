-- Your SQL goes here
-- 
CREATE TYPE role AS ENUM ('USER' , 'ADMIN');

CREATE TABLE room (
 id            VARCHAR(100) PRIMARY KEY,
 code          CHAR(9) NOT NULL UNIQUE ,
 name          VARCHAR(50),
 admin_id      VARCHAR(100),
 created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
 CONSTRAINT admin FOREIGN KEY (admin_id) REFERENCES users(id)
);


CREATE TABLE room_user(
id          VARCHAR(100) PRIMARY KEY,
user_id     VARCHAR(100) NOT NULL,
room_id     VARCHAR(100) NOT NULL,
role_assign role DEFAULT 'USER',
CONSTRAINT fk_room_user_user FOREIGN KEY (user_id) REFERENCES users(id),
CONSTRAINT fk_room_user_room FOREIGN KEY (room_id) REFERENCES room(id)
);

CREATE TABLE message(
id             VARCHAR(100) PRIMARY KEY,
text           TEXT NOT NULL,
created_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
user_id        VARCHAR(100) NOT NULL,
room_id        VARCHAR(100) NOT NULL,
CONSTRAINT fk_message_user FOREIGN KEY (user_id) REFERENCES users(id),
CONSTRAINT fk_message_room FOREIGN KEY (room_id) REFERENCES room(id)
);


