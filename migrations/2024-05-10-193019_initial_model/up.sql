-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE employees (
    id VARCHAR PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    full_name VARCHAR NOT NULL,
    country VARCHAR,
    join_date TIMESTAMP NOT NULL
);

CREATE TABLE projects (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL,
    admin_id VARCHAR NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (admin_id) REFERENCES employees(id)
);

CREATE TABLE onboardees (
    project_id UUID NOT NULL,
    employee_id VARCHAR NOT NULL,
    onboarding_date TIMESTAMP NOT NULL,
    PRIMARY KEY (project_id, employee_id),
    UNIQUE (project_id, employee_id),
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);