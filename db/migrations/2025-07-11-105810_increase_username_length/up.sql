-- Your SQL goes here
-- up.sql
ALTER TABLE users
  ALTER COLUMN user_name TYPE VARCHAR(150),
  ALTER COLUMN password TYPE VARCHAR(150);
