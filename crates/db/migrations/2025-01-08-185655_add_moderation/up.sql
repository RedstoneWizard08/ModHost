CREATE TYPE moderation_status AS ENUM ('pending', 'denied', 'approved', 'under_review');

CREATE TABLE IF NOT EXISTS moderation_queue (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    assigned_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    status moderation_status NOT NULL DEFAULT 'pending'
);

CREATE TABLE IF NOT EXISTS moderation_comment (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    is_moderator BOOLEAN NOT NULL DEFAULT FALSE,
    comment TEXT NOT NULL
);
