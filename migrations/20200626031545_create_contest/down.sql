-- This file should undo anything in `up.sql`
DROP TABLE match_records;
DROP TABLE tournaments;
DROP TABLE contest_item_descs;
DROP TABLE contest_items;
DROP TABLE contests;

DROP FUNCTION IF EXISTS set_cnt_win_lose();