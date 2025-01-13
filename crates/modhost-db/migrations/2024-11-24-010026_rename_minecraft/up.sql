ALTER TABLE package_versions RENAME COLUMN minecraft TO game_versions;
ALTER TABLE package_versions DROP COLUMN kubejs;
