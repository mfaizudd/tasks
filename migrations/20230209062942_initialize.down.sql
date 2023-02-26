-- Add down migration script here
DROP TABLE "assignment_scores";
DROP TABLE "assignments";
DROP TABLE "students";
DROP TABLE "cohorts";

DROP EXTENSION IF EXISTS "uuid-ossp";

