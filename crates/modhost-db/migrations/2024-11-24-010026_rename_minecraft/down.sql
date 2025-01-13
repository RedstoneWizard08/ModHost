ALTER TABLE package_versions RENAME COLUMN game_versions TO minecraft;
ALTER TABLE package_versions ADD COLUMN IF NOT EXISTS kubejs TEXT[] NOT NULL DEFAULT '{}';
