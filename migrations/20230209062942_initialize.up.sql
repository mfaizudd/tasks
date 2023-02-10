-- Add up migration script here
CREATE TYPE "user_type" AS ENUM ('email', 'google');
CREATE TYPE "user_role" AS ENUM ('admin', 'teacher', 'student');
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "user_accounts" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL DEFAULT '',
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "type" "user_type" NOT NULL,
    "role" "user_role" NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "user_password" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "user_id" uuid NOT NULL REFERENCES "user_accounts"("id") ON DELETE CASCADE,
    "password" VARCHAR(255) NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "classrooms" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "user_id" uuid NOT NULL REFERENCES "user_accounts"("id") ON DELETE RESTRICT,
    "invite_code" VARCHAR(255) NOT NULL UNIQUE,
    "name" VARCHAR(255) NOT NULL,
    "description" VARCHAR(255) NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "students" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "number" VARCHAR(255) NOT NULL,
    "user_id" uuid NULL REFERENCES "user_accounts"("id") ON DELETE CASCADE,
    "classroom_id" uuid NOT NULL REFERENCES "classrooms"("id") ON DELETE CASCADE,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);


CREATE TABLE IF NOT EXISTS "tasks" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "user_id" uuid NOT NULL REFERENCES "user_accounts"("id") ON DELETE RESTRICT,
    "classroom_id" uuid NOT NULL REFERENCES "classrooms"("id") ON DELETE RESTRICT,
    "title" VARCHAR(255) NOT NULL,
    "description" VARCHAR(255) NOT NULL,
    "score_max" integer NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "task_assignments" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "student_id" uuid NOT NULL REFERENCES "students"("id") ON DELETE CASCADE,
    "task_id" uuid NOT NULL REFERENCES "tasks"("id") ON DELETE CASCADE,
    "score" integer NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "teacher_invite_codes" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "code" VARCHAR(255) NOT NULL UNIQUE,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);
