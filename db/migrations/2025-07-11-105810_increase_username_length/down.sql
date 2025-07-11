-- This file should undo anything in `up.sql`
-- down.sql
ALTER TABLE users
  ALTER COLUMN user_name TYPE VARCHAR(50),
  ALTER COLUMN password TYPE VARCHAR(50);
