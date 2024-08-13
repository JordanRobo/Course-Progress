-- Your SQL goes here
CREATE TABLE assignments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    assessment_type TEXT NOT NULL,
    unit TEXT NOT NULL,
    submitted INTEGER NOT NULL
);