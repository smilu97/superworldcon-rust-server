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
        contest_id -> Uuid,
    }
}

table! {
    contest_round_matches (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        match_size -> Int4,
        win_id -> Uuid,
        lose_id -> Uuid,
        contest_round_id -> Uuid,
    }
}

table! {
    contest_rounds (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Uuid,
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
        last_action -> Nullable<Timestamp>,
    }
}

joinable!(contest_item_descs -> contest_items (contest_item_id));
joinable!(contest_items -> contests (contest_id));
joinable!(contest_round_matches -> contest_rounds (contest_round_id));
joinable!(contest_rounds -> contests (contest_id));
joinable!(contest_rounds -> users (user_id));

allow_tables_to_appear_in_same_query!(
    contest_item_descs,
    contest_items,
    contest_round_matches,
    contest_rounds,
    contests,
    users,
);
