-- Add migration script here
CREATE TABLE IF NOT EXISTS djots (
    id uuid PRIMARY KEY NOT NULL,
    html bytea NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);