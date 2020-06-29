use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::tournaments;
use crate::schema::match_records;

#[derive(Queryable, Deserialize, Serialize, Identifiable)]
#[table_name = "tournaments"]
pub struct Tournament {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
    pub contest_id: Uuid,
}

#[derive(Queryable, Deserialize, Serialize, Identifiable)]
#[table_name = "match_records"]
pub struct MatchRecord {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub size: i32,
    pub win_id: Uuid,
    pub lose_id: Uuid,
    pub tournament_id: Uuid,
}

#[derive(Deserialize, Insertable)]
#[table_name = "tournaments"]
pub struct NewTournament {
    pub user_id: Uuid,
    pub contest_id: Uuid,
}

#[derive(Deserialize, Insertable)]
#[table_name = "match_records"]
pub struct NewMatchRecord {
    pub size: i32,
    pub win_id: Uuid,
    pub lose_id: Uuid,
    pub tournament_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct TournamentInput {
    pub user_id: Uuid,
    pub contest_id: Uuid,
    pub records: Vec<MatchRecordInput>,
}

#[derive(Deserialize, Serialize)]
pub struct MatchRecordInput {
    pub size: i32,
    pub win_id: Uuid,
    pub lose_id: Uuid,
}
