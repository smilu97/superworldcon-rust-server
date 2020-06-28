-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE contests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    title VARCHAR(128) NOT NULL,
    num_items INTEGER DEFAULT 0 NOT NULL
);
SELECT diesel_manage_updated_at('contests');

CREATE TABLE contest_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    title VARCHAR(128) NOT NULL,

    contest_id UUID NOT NULL,
    FOREIGN KEY (contest_id) REFERENCES contests (id) ON DELETE CASCADE
);
SELECT diesel_manage_updated_at('contest_items');

CREATE TABLE contest_item_descs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    title VARCHAR(128) NOT NULL,
    desc_type VARCHAR(8) DEFAULT 'image' NOT NULL,
    url VARCHAR(2083) NOT NULL,

    contest_item_id UUID NOT NULL,
    FOREIGN KEY (contest_item_id) REFERENCES contest_items (id) ON DELETE CASCADE
);
SELECT diesel_manage_updated_at('contest_item_descs');

CREATE TABLE contest_rounds (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,

    user_id UUID NOT NULL,
    contest_id UUID NOT NULL,
    FOREIGN KEY (contest_id) REFERENCES contests (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL
);
SELECT diesel_manage_updated_at('contest_rounds');

CREATE TABLE contest_round_matches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,

    match_size INTEGER NOT NULL,
    win_id UUID NOT NULL,
    lose_id UUID NOT NULL,
    FOREIGN KEY (win_id) REFERENCES contest_items (id),
    FOREIGN KEY (lose_id) REFERENCES contest_items (id),

    contest_round_id UUID NOT NULL,
    FOREIGN KEY (contest_round_id) REFERENCES contest_rounds (id) ON DELETE CASCADE
);
SELECT diesel_manage_updated_at('contest_round_matches');
