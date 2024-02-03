-- FOR DEVELOPMENT ONLY - Brute force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE usename = 'app_dev' OR datname = 'app_db';
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_dev;

-- FOR DEVELOPMENT ONLY - Dev only password (for local dev and unit test)
CREATE USER app_dev PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db owner app_dev ENCODING = 'UTF-8';
