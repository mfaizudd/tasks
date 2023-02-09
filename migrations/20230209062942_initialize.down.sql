-- Add down migration script here
DROP TABLE IF EXISTS "task_assignments";
DROP TABLE IF EXISTS "tasks";
DROP TABLE IF EXISTS "classrooms";
DROP TABLE IF EXISTS "user_password";
DROP TABLE IF EXISTS "user_accounts";

DROP EXTENSION IF EXISTS "uuid-ossp";
DROP TYPE IF EXISTS "user_role";
DROP TYPE IF EXISTS "user_type";

