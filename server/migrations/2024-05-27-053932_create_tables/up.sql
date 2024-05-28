-- Your SQL goes here
-- Create the Authors table
CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    bio TEXT,
    profile_picture VARCHAR(255)
);

-- Create the Tags table
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL
);

-- Create the Posts table
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    feature_image VARCHAR(255),
    excerpt TEXT,
    published BOOLEAN DEFAULT false,
    author_id INTEGER REFERENCES authors(id),
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
);

-- Create the PostTags table (many-to-many relationship)
CREATE TABLE post_tags (
    post_id INTEGER REFERENCES posts(id),
    tag_id INTEGER REFERENCES tags(id),
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);