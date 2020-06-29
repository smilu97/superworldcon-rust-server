-- delete from match_records where 1=1;
-- delete from tournaments where 1=1;
-- delete from contest_item_descs where title like 'sample contest item description %';
-- delete from contest_items where title like 'sample contest item %';
delete from contests where title = 'sample contest';
delete from users where email = 'admin@mail.com';
