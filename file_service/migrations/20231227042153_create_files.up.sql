CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) UNIQUE NOT NULL,
    content_type VARCHAR(128) NOT NULL,
    extension VARCHAR(8),
    internal_name VARCHAR(64) NOT NULL
)