CREATE TABLE IF NOT EXISTS ItemFieldLinks(
    id SERIAL PRIMARY KEY NOT NULL,
    iid INTEGER NOT NULL REFERENCES Items(id),
    fid INTEGER NOT NULL REFERENCES Fields(id),
    priority TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
