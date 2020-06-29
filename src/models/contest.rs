use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::contests;
use crate::schema::contest_items;
use crate::schema::contest_item_descs;

#[derive(Queryable, Deserialize, Serialize, Identifiable)]
#[table_name = "contests"]
pub struct Contest {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub title: String,
    pub num_items: i32,
    pub visible: bool,
}

#[derive(Queryable, Deserialize, Serialize, Identifiable)]
#[table_name = "contest_items"]
pub struct ContestItem {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub title: String,
    pub count_win: i64,
    pub count_lose: i64,
    pub contest_id: Uuid,
}

#[derive(Queryable, Deserialize, Serialize, Identifiable, Clone)]
#[table_name = "contest_item_descs"]
pub struct ContestItemDesc {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub title: String,
    pub desc_type: String,
    pub url: String,
    pub contest_item_id: Uuid,
}

#[derive(Deserialize, Insertable)]
#[table_name = "contests"]
pub struct NewContest {
    pub title: String,
    pub num_items: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "contest_items"]
pub struct NewContestItem {
    pub title: String,
    pub contest_id: Uuid,
}

#[derive(Deserialize, Insertable)]
#[table_name = "contest_item_descs"]
pub struct NewContestItemDesc {
    pub title: String,
    pub desc_type: String,
    pub url: String,
    pub contest_item_id: Uuid,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DescInput {
    pub title: String,
    pub desc_type: String,
    pub url: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ItemInput {
    pub title: String,
    pub descriptions: Vec<DescInput>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ContestInput {
    pub title: String,
    pub num_items: i32,
    pub items: Vec<ItemInput>,
}
