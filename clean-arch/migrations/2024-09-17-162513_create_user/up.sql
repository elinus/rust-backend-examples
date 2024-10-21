-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    phone VARCHAR NOT NULL,
    address VARCHAR NOT NULL
);

CREATE INDEX index_users_on_email ON users(email);
