-- Your SQL goes here

CREATE TABLE employees (
    id UUID PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    full_name VARCHAR NOT NULL,
    country VARCHAR,
    join_date TIMESTAMP NOT NULL
);