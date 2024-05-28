-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS posts CASCADE;
DROP TABLE IF EXISTS post_tags CASCADE;
DROP TABLE IF EXISTS tags CASCADE;
DROP TABLE IF EXISTS authors CASCADE;