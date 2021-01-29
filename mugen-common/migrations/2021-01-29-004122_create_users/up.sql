CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    email TEXT NOT NULL,
    first_name TEXT DEFAULT NULL,
    last_name TEXT DEFAULT NULL
);
