-- Create the authors table
CREATE TABLE authors (
  id VARCHAR(20) PRIMARY KEY,
  name VARCHAR(255) DEFAULT '',
  email VARCHAR(255) DEFAULT '',
  bio TEXT DEFAULT '',
  profile_picture VARCHAR(255) DEFAULT ''
);

-- Create the tags table
CREATE TABLE tags (
  id VARCHAR(20) PRIMARY KEY,
  name VARCHAR(255) DEFAULT '',
  slug VARCHAR(255) DEFAULT ''
);

-- Create the posts table
CREATE TABLE posts (
  id VARCHAR(20) PRIMARY KEY,
  title VARCHAR(255) DEFAULT '',
  slug VARCHAR(255) DEFAULT '',
  content TEXT DEFAULT '',
  feature_image VARCHAR(255) DEFAULT '',
  excerpt TEXT DEFAULT '',
  published BOOLEAN DEFAULT FALSE,
  author_id VARCHAR(20),
  FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
);

-- Create the post_tags table for the many-to-many relationship between posts and tags
CREATE TABLE post_tags (
  post_id VARCHAR(20),
  tag_id VARCHAR(20),
  PRIMARY KEY (post_id, tag_id),
  FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);