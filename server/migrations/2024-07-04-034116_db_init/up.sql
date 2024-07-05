-- Your SQL goes here
-- Create the authors table
CREATE TABLE authors (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT DEFAULT '',
    email TEXT DEFAULT '',
    bio TEXT DEFAULT '',
    profile_picture TEXT DEFAULT ''
);

-- Create the tags table
CREATE TABLE tags (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT DEFAULT '',
    slug TEXT DEFAULT ''
);

-- Create the posts table
CREATE TABLE posts (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT DEFAULT '',
    slug TEXT DEFAULT '',
    content TEXT DEFAULT '',
    feature_image TEXT DEFAULT '',
    excerpt TEXT DEFAULT '',
    published INTEGER DEFAULT 0,
    author_id TEXT,
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
);

-- Create the post_tags table for the many-to-many relationship between posts and tags
CREATE TABLE post_tags (
    post_id TEXT,
    tag_id TEXT,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);