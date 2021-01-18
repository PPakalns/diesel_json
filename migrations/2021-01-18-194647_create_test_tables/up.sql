-- Your SQL goes here

CREATE TABLE jsonb_test (
    id SERIAL PRIMARY KEY,
    nullable JSONB,
    not_nullable JSONB NOT NULL
);
