-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE contests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    title VARCHAR(128) NOT NULL,
    num_items INTEGER DEFAULT 0 NOT NULL,
    visible BOOLEAN DEFAULT TRUE NOT NULL
);
SELECT diesel_manage_updated_at('contests');

CREATE TABLE contest_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    title VARCHAR(128) NOT NULL,
    
    count_win  BIGINT DEFAULT 0 NOT NULL,
    count_lose BIGINT DEFAULT 0 NOT NULL,

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

CREATE TABLE tournaments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,

    user_id UUID,
    contest_id UUID NOT NULL,
    FOREIGN KEY (contest_id) REFERENCES contests (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL
);
SELECT diesel_manage_updated_at('tournaments');

CREATE UNIQUE INDEX tour_contest_user_idx ON tournaments (user_id, contest_id);

CREATE TABLE match_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,

    size INTEGER NOT NULL,
    win_id UUID NOT NULL,
    lose_id UUID NOT NULL,
    FOREIGN KEY (win_id)  REFERENCES contest_items (id) ON DELETE CASCADE,
    FOREIGN KEY (lose_id) REFERENCES contest_items (id) ON DELETE CASCADE,

    tournament_id UUID NOT NULL,
    FOREIGN KEY (tournament_id) REFERENCES tournaments (id) ON DELETE CASCADE
);
SELECT diesel_manage_updated_at('match_records');

CREATE OR REPLACE FUNCTION set_cnt_win_lose() RETURNS trigger AS $trigger_set_cnt_win_lose$
BEGIN
    UPDATE contest_items
        SET count_win = (
            SELECT count(*) FROM match_records WHERE win_id = NEW.win_id
        )
        WHERE id = NEW.win_id;
    UPDATE contest_items
        SET count_lose = (
            SELECT count(*) FROM match_records WHERE lose_id = NEW.lose_id
        )
        WHERE id = NEW.lose_id;
    RETURN NULL;
END;
$trigger_set_cnt_win_lose$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_set_cnt_win_lose
    AFTER INSERT OR UPDATE OR DELETE ON match_records
    FOR EACH ROW EXECUTE PROCEDURE set_cnt_win_lose();
