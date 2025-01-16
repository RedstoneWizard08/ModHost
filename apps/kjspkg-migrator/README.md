# KJSPKG Migrator

A tool for migrating packages from the old [KJSPKG](https://github.com/Modern-Modpacks/kjspkg) to a new ModHost-based instance.

## Usage

1. Make sure to set `MIGRATOR_TOKEN` in `.env`. It's just a GitHub personal access token - it doesn't actually need permissions. It's only used for ratelimit purposes.
2. Clear the database: `diesel migration redo -a`
3. Run it: `cargo run --bin kjspkg-migrator`

Unfortunately, due to the nature of how old KJSPKG was set up, a couple of packages that no longer exist on GitHub yet still exist in the manifest won't be migrated. You'll see their error messages in the tool's output.
