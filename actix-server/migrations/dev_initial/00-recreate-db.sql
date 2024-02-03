-- FOR DEVELOPMENT ONLY - Brute force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE usename = 'owl_project' OR datname = 'owl_db';
DROP DATABASE IF EXISTS owl_db;
DROP USER IF EXISTS owl_project;

-- FOR DEVELOPMENT ONLY - Dev only password (for local dev and unit test)
CREATE USER owl_project PASSWORD 'dev_only_pwd';
CREATE DATABASE owl_db owner owl_project ENCODING = 'UTF-8';
