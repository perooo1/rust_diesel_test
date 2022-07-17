-- Your SQL goes here
CREATE TABLE posts(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    pubblished BOOLEAN NOT NULL DEFAULT 'f'
)