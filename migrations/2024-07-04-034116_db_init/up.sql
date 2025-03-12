CREATE TABLE IF NOT EXISTS post_tags (
  id TEXT NOT NULL PRIMARY KEY,
  post_id TEXT NOT NULL,
  tag_id TEXT NOT NULL,
  FOREIGN KEY (post_id) REFERENCES posts (post_id),
  FOREIGN KEY (tag_id) REFERENCES tags (tag_id),
  UNIQUE(post_id, tag_id) 
);

CREATE INDEX IF NOT EXISTS public_idx_post_tags_post_id_tag_id ON post_tags (post_id, tag_id);

CREATE TABLE IF NOT EXISTS media (
  media_id TEXT NOT NULL PRIMARY KEY,
  file_name TEXT NOT NULL,
  file_type TEXT NOT NULL,
  file_size TEXT NOT NULL,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS pass_reset_tokens (
  token_id TEXT NOT NULL PRIMARY KEY,
  user_id TEXT NOT NULL,
  token_value TEXT NOT NULL,
  token_expiry DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS posts (
  post_id TEXT NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  excerpt TEXT,
  content TEXT,
  author_id TEXT NOT NULL,
  feature_image TEXT,
  status TEXT,
  published_at DATETIME,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL,
  FOREIGN KEY (author_id) REFERENCES authors (author_id),
  FOREIGN KEY (feature_image) REFERENCES media (media_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS public_idx_posts_slug ON posts (slug);

CREATE TABLE IF NOT EXISTS roles (
  role_id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS site_analytics (
  analytics_id TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS post_categories (
  id TEXT NOT NULL PRIMARY KEY,
  post_id TEXT NOT NULL,
  category_id TEXT NOT NULL,
  FOREIGN KEY (post_id) REFERENCES posts (post_id),
  FOREIGN KEY (category_id) REFERENCES categories (category_id)
);

CREATE TABLE IF NOT EXISTS related_posts (
  id TEXT NOT NULL PRIMARY KEY,
  post_id TEXT NOT NULL,
  related_post_id TEXT NOT NULL,
  strength INTEGER NOT NULL,
  FOREIGN KEY (post_id) REFERENCES posts (post_id),
  FOREIGN KEY (related_post_id) REFERENCES posts (post_id)
);

CREATE TABLE IF NOT EXISTS audit_logs (
  log_id TEXT NOT NULL PRIMARY KEY,
  user_id TEXT NOT NULL,
  activity_type TEXT NOT NULL,
  timestamp DATETIME NOT NULL,
  description TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (user_id)
);

CREATE TABLE IF NOT EXISTS tags (
  tag_id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  slug TEXT NOT NULL UNIQUE
);

CREATE UNIQUE INDEX IF NOT EXISTS public_idx_tags_name ON tags (name);
CREATE UNIQUE INDEX IF NOT EXISTS public_idx_tags_slug ON tags (slug);

CREATE TABLE IF NOT EXISTS users (
  user_id TEXT NOT NULL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  first_name TEXT NOT NULL,
  last_name TEXT,
  pass_hash TEXT NOT NULL,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL,
  last_login DATETIME NOT NULL
);

CREATE INDEX IF NOT EXISTS index_1 ON users (email, pass_hash);

CREATE TABLE IF NOT EXISTS categories (
  category_id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS user_roles (
  id TEXT NOT NULL PRIMARY KEY,
  user_id TEXT NOT NULL,
  role_id TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (user_id),
  FOREIGN KEY (role_id) REFERENCES roles (role_id)
);

CREATE TABLE IF NOT EXISTS authors (
  author_id TEXT NOT NULL PRIMARY KEY,
  user_id TEXT NOT NULL UNIQUE, 
  slug TEXT NOT NULL UNIQUE,
  bio TEXT,
  profile_picture TEXT,
  FOREIGN KEY (user_id) REFERENCES users (user_id),
  FOREIGN KEY (profile_picture) REFERENCES media (media_id) 
);

CREATE UNIQUE INDEX IF NOT EXISTS public_idx_users_slug ON authors (slug);