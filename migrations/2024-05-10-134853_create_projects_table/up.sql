-- Your SQL goes here

CREATE TABLE projects (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    admin_id UUID NOT NULL,
    FOREIGN KEY (admin_id) REFERENCES employees(id)
);
