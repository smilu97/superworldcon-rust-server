table! {
    contest_item_descs (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        desc_type -> Varchar,
        url -> Varchar,
        contest_item_id -> Uuid,
    }
}

table! {
    contest_items (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        count_win -> Int8,
        count_lose -> Int8,
        contest_id -> Uuid,
    }
}

table! {
    contests (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        num_items -> Int4,
        visible -> Bool,
    }
}

table! {
    match_records (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        size -> Int4,
        win_id -> Uuid,
        lose_id -> Uuid,
        tournament_id -> Uuid,
    }
}

table! {
    tournaments (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Nullable<Uuid>,
        contest_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
        password_hash -> Bytea,
        current_auth_token -> Nullable<Varchar>,
        last_login -> Nullable<Timestamp>,
    }
}

joinable!(contest_item_descs -> contest_items (contest_item_id));
joinable!(contest_items -> contests (contest_id));
joinable!(match_records -> tournaments (tournament_id));
joinable!(tournaments -> contests (contest_id));
joinable!(tournaments -> users (user_id));

allow_tables_to_appear_in_same_query!(
    contest_item_descs,
    contest_items,
    contests,
    match_records,
    tournaments,
    users,
);
