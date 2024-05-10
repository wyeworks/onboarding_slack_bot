-- Your SQL goes here

CREATE TABLE onboardees (
    project_id UUID NOT NULL,
    employee_id UUID NOT NULL,
    onboarding_date DATE,
    PRIMARY KEY (project_id, employee_id),
    UNIQUE (project_id, employee_id),
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);
