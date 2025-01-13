ALTER TABLE packages RENAME TO projects;
ALTER TABLE package_versions RENAME TO project_versions;
ALTER TABLE package_version_refs RENAME TO project_version_refs;
ALTER TABLE package_relations RENAME TO project_relations;
ALTER TABLE package_authors RENAME TO project_authors;

ALTER TABLE project_versions RENAME COLUMN package TO project;
ALTER TABLE project_authors RENAME COLUMN package TO project;
ALTER TABLE project_relations RENAME COLUMN package TO project;
ALTER TABLE gallery_images RENAME COLUMN package TO project;
