-- This file should undo anything in `up.sql`
-- Down Migration (to rollback)
DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS user_status;
