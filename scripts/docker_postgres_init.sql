CREATE USER diesel_test WITH PASSWORD 'diesel_test' CREATEDB;
CREATE DATABASE diesel_test
    WITH
    OWNER = diesel_test
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;
