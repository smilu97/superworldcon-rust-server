DO $$
DECLARE
    l_admin_id UUID;
    l_contest_id UUID;
    l_contest_item_id_1 UUID;
    l_contest_item_id_2 UUID;
    l_contest_round_id UUID;
BEGIN
    INSERT INTO users(email, password_hash) VALUES ('admin@mail.com', 'todo');
    SELECT id INTO l_admin_id FROM users WHERE email = 'admin@mail.com';
    INSERT INTO contests(title, num_items) VALUES ('sample contest', 2);
    SELECT id INTO l_contest_id FROM contests WHERE title = 'sample contest';
    INSERT INTO contest_items(title, contest_id) VALUES 
        ('sample contest item 1', l_contest_id),
        ('sample contest item 2', l_contest_id);
    SELECT id INTO l_contest_item_id_1 FROM contest_items WHERE title = 'sample contest item 1';
    SELECT id INTO l_contest_item_id_2 FROM contest_items WHERE title = 'sample contest item 2';
    INSERT INTO contest_item_descs(title, desc_type, url, contest_item_id) VALUES
        ('sample contest item description 1', 'text', 'sample url 1', l_contest_item_id_1),
        ('sample contest item description 2', 'text', 'sample url 2', l_contest_item_id_2);
    INSERT INTO contest_rounds(user_id, contest_id) VALUES (l_admin_id, l_contest_id);
    SELECT id INTO l_contest_round_id FROM contest_rounds WHERE user_id = l_admin_id AND contest_id = l_contest_id;
    INSERT INTO contest_round_matches(match_size, win_id, lose_id, contest_round_id) VALUES
        (2, l_contest_item_id_1, l_contest_item_id_2, l_contest_round_id);
END $$;