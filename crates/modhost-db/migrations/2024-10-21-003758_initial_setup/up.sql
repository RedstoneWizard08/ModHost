CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    github_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS user_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS packages (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    readme TEXT NOT NULL,
    description TEXT NOT NULL,
    views INTEGER NOT NULL DEFAULT 0,
    source TEXT, -- repo
    issues TEXT,
    wiki TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS package_versions (
    id SERIAL PRIMARY KEY,
    package INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    version_number TEXT NOT NULL,
    file_id TEXT NOT NULL, -- This is the UUID of the version's file.
    changelog TEXT,
    kubejs TEXT[] NOT NULL DEFAULT '{}',
    loaders TEXT[] NOT NULL DEFAULT '{}',
    minecraft TEXT[] NOT NULL DEFAULT '{}',
    downloads INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = now(); 
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_package_versions_updated_at BEFORE UPDATE
ON package_versions FOR EACH ROW EXECUTE PROCEDURE 
update_updated_at_column();

CREATE TRIGGER update_packages_updated_at BEFORE UPDATE
ON packages FOR EACH ROW EXECUTE PROCEDURE 
update_updated_at_column();

-- How to deal with diesel.rs shenanigains, part 1
CREATE TABLE IF NOT EXISTS package_version_refs (
    value INTEGER NOT NULL REFERENCES package_versions(id) ON DELETE CASCADE,
    PRIMARY KEY(value)
);

CREATE TABLE IF NOT EXISTS package_relations (
    package INTEGER NOT NULL REFERENCES package_versions(id) ON DELETE CASCADE,
    dependency INTEGER NOT NULL REFERENCES package_version_refs(value) ON DELETE CASCADE,
    -- The relation kind. 0 = dependency, 1 = incompatibility
    kind INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY(package, dependency, kind)
);

CREATE TABLE IF NOT EXISTS package_authors (
    package INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY(package, user_id)
);
