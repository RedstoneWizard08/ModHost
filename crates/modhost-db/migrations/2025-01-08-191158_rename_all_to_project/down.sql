ALTER TABLE projects RENAME TO packages;
ALTER TABLE project_versions RENAME TO package_versions;
ALTER TABLE project_version_refs RENAME TO package_version_refs;
ALTER TABLE project_relations RENAME TO package_relations;
ALTER TABLE project_authors RENAME TO package_authors;

ALTER TABLE package_versions RENAME COLUMN project TO package;
ALTER TABLE package_authors RENAME COLUMN project TO package;
ALTER TABLE package_relations RENAME COLUMN project TO package;
ALTER TABLE gallery_images RENAME COLUMN project TO package;
