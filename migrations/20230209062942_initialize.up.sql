-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "cohorts" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "students" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "number" VARCHAR(255) NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "cohort_students" (
    "cohort_id" uuid NOT NULL REFERENCES "cohorts"("id") ON DELETE RESTRICT,
    "student_id" uuid NOT NULL REFERENCES "students"("id") ON DELETE RESTRICT,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now(),
    PRIMARY KEY ("cohort_id", "student_id")
);

CREATE TABLE IF NOT EXISTS "assignments" (
    "id" uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    "cohort_id" uuid NOT NULL REFERENCES "cohorts"("id") ON DELETE RESTRICT,
    "name" VARCHAR(255) NOT NULL,
    "description" VARCHAR(255) NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "assignment_scores" (
    "assignment_id" uuid NOT NULL REFERENCES "assignments"("id") ON DELETE CASCADE,
    "student_id" uuid NOT NULL REFERENCES "students"("id") ON DELETE CASCADE,
    "score" integer NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now(),
    PRIMARY KEY ("assignment_id", "student_id")
);

