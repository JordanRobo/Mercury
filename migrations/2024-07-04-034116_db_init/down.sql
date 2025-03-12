-- Drop indexes first
DROP INDEX IF EXISTS public_idx_users_slug;
DROP INDEX IF EXISTS public_idx_tags_slug;
DROP INDEX IF EXISTS public_idx_tags_name;
DROP INDEX IF EXISTS public_idx_posts_slug;
DROP INDEX IF EXISTS public_idx_post_tags_post_id_tag_id;
DROP INDEX IF EXISTS index_1;

-- Drop tables with foreign key dependencies first
DROP TABLE IF EXISTS authors;
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS post_categories;
DROP TABLE IF EXISTS post_tags;
DROP TABLE IF EXISTS related_posts;
DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS pass_reset_tokens;
DROP TABLE IF EXISTS posts;

-- Drop tables with fewer dependencies
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS site_analytics;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS media;