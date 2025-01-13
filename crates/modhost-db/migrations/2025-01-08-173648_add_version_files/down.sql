DROP TABLE IF EXISTS version_files;
ALTER TABLE package_versions ADD file_id TEXT NOT NULL DEFAULT '';
