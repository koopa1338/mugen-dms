CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMPTZ,
    filetype VARCHAR(12),
    version INT NOT NULL DEFAULT 1,
    size BIGINT NOT NULL
);