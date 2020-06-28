use rocket::get;
use rocket::http::RawStr;
use rocket_contrib::json;
use serde_json::Value;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use superslice::*;

use crate::database::DbConn;
use crate::responses::{self, APIResponse};

use crate::schema::contests;
use crate::schema::contest_items;
use crate::schema::contest_item_descs;

#[derive(Queryable, Deserialize, Serialize, Identifiable)]
#[table_name = "contests"]
struct Contest {
    id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    title: String,
    num_items: i32,
    visible: bool,
}

#[derive(Queryable, Deserialize, Serialize, Identifiable)]
#[table_name = "contest_items"]
struct ContestItem {
    id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    title: String,
    count_win: i64,
    count_lose: i64,
    contest_id: Uuid,
}

#[derive(Queryable, Deserialize, Serialize, Identifiable, Clone)]
#[table_name = "contest_item_descs"]
struct ContestItemDesc {
    id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    title: String,
    desc_type: String,
    url: String,
    contest_item_id: Uuid,
}

#[get("/contests")]
pub fn handle_get_contests(db: DbConn) -> APIResponse {
    use crate::schema::contests::dsl::*;
    let r_contests = contests
        .load::<Contest>(&*db)
        .expect("Error loading contests");
    
    responses::ok().data(json!(r_contests))
}

fn find_children<T: Clone + serde::Serialize>(children: &Vec<T>, pids: &Vec<Uuid>, id: &Uuid) -> Value {
    let mut tmp_stack = Vec::<T>::new();
    let lo = pids.lower_bound(id);
    let up = pids.upper_bound(id);
    for i in lo..up {
        tmp_stack.push(children[i].clone());
    }
    serde_json::json!(tmp_stack)
}

#[get("/contest/<s_cid>")]
pub fn handle_get_contest(s_cid: &RawStr, db: DbConn) -> APIResponse {
    use crate::schema::contests::dsl::*;
    use crate::schema::contest_items::dsl::*;
    use crate::schema::contest_item_descs::dsl::*;

    let cid = Uuid::from_str(s_cid.as_str())
        .expect("Error parsing contest id");
    let r_contest = contests
        .filter(crate::schema::contests::id.eq(&cid))
        .get_result::<Contest>(&*db)
        .expect("Error loading contest");
    
    let r_items = contest_items
        .filter(contest_id.eq(&r_contest.id))
        .load::<ContestItem>(&*db)
        .expect("Error loading contest items");
    
    let item_ids: Vec<Uuid> = (&r_items).into_iter().map(|x| -> Uuid { x.id }).collect();
    let mut r_descs = contest_item_descs
        .filter(contest_item_id.eq_any(item_ids))
        .load::<ContestItemDesc>(&*db)
        .expect("Error loading contest item descriptions");
    r_descs.sort_unstable_by_key(|x| -> Uuid { x.contest_item_id });
    let desc_pids: Vec<Uuid> = (&r_descs).into_iter().map(|x| -> Uuid { x.contest_item_id }).collect();

    let j_items: Vec<Value> = r_items.into_iter().map(|x| -> Value {
        let mut j_item = serde_json::json!(x);
        j_item["descriptions"] = find_children(&r_descs, &desc_pids, &x.id);
        j_item
    }).collect();

    let mut j_contest = serde_json::json!(r_contest);
    j_contest["items"] = serde_json::json!(j_items);
    
    responses::ok().data(json!(j_contest))
}
