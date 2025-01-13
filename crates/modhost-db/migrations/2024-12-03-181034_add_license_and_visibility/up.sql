CREATE TYPE Visibility as ENUM ('public', 'private', 'unlisted');

ALTER TABLE packages ADD license TEXT;
ALTER TABLE packages ADD visibility Visibility NOT NULL DEFAULT 'public';
