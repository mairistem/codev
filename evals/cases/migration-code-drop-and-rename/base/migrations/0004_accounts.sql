CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    user_name TEXT NOT NULL,
    legacy_email TEXT
);
