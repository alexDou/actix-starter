CREATE USER alx_admin WITH PASSWORD 'Quuth1si8pha' SUPERUSER;
CREATE DATABASE "actix-starter";
CREATE USER alx_actix WITH PASSWORD 'aeph1Op6o104';

GRANT ALL PRIVILEGES ON DATABASE "actix-starter" TO alx_actix;

-- PostgreSQL 15+ Requirement. Connect to the new database to grant schema permissions
\c "actix-starter"

-- Grant schema-level permissions so the app user can create tables
GRANT ALL ON SCHEMA public TO alx_actix;
