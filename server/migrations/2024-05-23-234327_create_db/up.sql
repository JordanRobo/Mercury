-- Your SQL goes here
CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    firstName VARCHAR NOT NULL,
    lastName VARCHAR NOT NULL,
    email TEXT NOT NULL
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    authorID INT NOT NULL,
    FOREIGN KEY (authorID) REFERENCES authors(id),
    published BOOLEAN DEFAULT FALSE
);

INSERT INTO authors (firstName, lastName, email) VALUES ('John', 'Doe', 'john.doe@mail.com');
INSERT INTO posts (title, content, authorID) VALUES ('Hello World', '<p>This is my first post<p>', 1);