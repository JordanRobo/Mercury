-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    firstName VARCHAR NOT NULL,
    lastName VARCHAR NOT NULL,
    email TEXT NOT NULL
);

INSERT INTO users (firstName, lastName, email) VALUES ('John', 'Doe', 'john@mail.com');