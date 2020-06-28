CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    email VARCHAR(120) UNIQUE NOT NULL,
    password_hash BYTEA NOT NULL,
    current_auth_token VARCHAR(32),
    last_action TIMESTAMP
);
SELECT diesel_manage_updated_at('users');

CREATE UNIQUE INDEX email_idx ON users(email);
CREATE UNIQUE INDEX current_auth_token_idx ON users(current_auth_token);
