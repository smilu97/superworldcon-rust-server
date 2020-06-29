use rocket::{get, post};
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::json;
use uuid::Uuid;
use std::str::FromStr;
use diesel::prelude::*;

use crate::database::DbConn;
use crate::responses::{self, APIResponse};
use crate::utils;

use crate::models::contest::Contest;
use crate::models::contest::ContestItem;
use crate::models::contest::ContestItemDesc;

use crate::models::contest::NewContest;
use crate::models::contest::NewContestItem;
use crate::models::contest::NewContestItemDesc;

use crate::models::contest::ContestInput;

#[get("/contests")]
pub fn handle_get_contests(db: DbConn) -> APIResponse {
    use crate::schema::contests::dsl::*;
    let r_contests = contests
        .load::<Contest>(&*db)
        .expect("Error loading contests");
    
    responses::ok().data(r_contests)
}

#[get("/contest/<s_cid>")]
pub fn handle_get_contest(s_cid: &RawStr, db: DbConn) -> APIResponse {
    use crate::schema::contests::dsl::*;
    use crate::schema::contest_items::dsl::*;
    use crate::schema::contest_item_descs::dsl::*;

    let cid_result = Uuid::from_str(s_cid.as_str());
    if cid_result.is_err() {
        return responses::unprocessable_entity(json!("Failed to parse contest id"));
    }
    let cid = cid_result.unwrap();
        
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

    let mut j_contest = json!(r_contest); 
    j_contest["items"] = utils::json::join_association(
        &r_items,
        |x| { x.id },
        &mut r_descs,
        |x| { x.contest_item_id },
        "descriptions"
    );
    
    responses::ok().data(j_contest)
}

#[post("/contest", data = "<in_contest>", format = "application/json")]
pub fn handle_post_contest(in_contest: Json<ContestInput>, db: DbConn) -> APIResponse {
    use crate::schema::contests::dsl::*;
    use crate::schema::contest_items::dsl::*;
    use crate::schema::contest_item_descs::dsl::*;

    let new_contest = NewContest {
        title: in_contest.title.clone(),
        num_items: in_contest.num_items,
    };
    let res_contest = diesel::insert_into(contests)
        .values(new_contest)
        .get_result::<Contest>(&*db)
        .expect("Error inserting contest");

    let new_items: Vec<NewContestItem> = (&in_contest.items).into_iter().map(|x| -> NewContestItem {
        NewContestItem {
            contest_id: res_contest.id,
            title: x.title.clone(),
        }
    }).collect();
    let res_items = diesel::insert_into(contest_items)
        .values(new_items)
        .get_results::<ContestItem>(&*db)
        .expect("Error inserting contest items");
    
    let v2_new_descs: Vec<Vec<NewContestItemDesc>> = (&in_contest.items).into_iter()
        .enumerate().map(|(ci, c)| -> Vec<NewContestItemDesc> {
        (&c.descriptions).into_iter().map(|x| -> NewContestItemDesc {
            NewContestItemDesc {
                contest_item_id: res_items[ci].id,
                title: x.title.clone(),
                desc_type: x.desc_type.clone(),
                url: x.url.clone(),
            }
        }).collect()
    }).collect();
    let new_descs: Vec<NewContestItemDesc> = v2_new_descs.into_iter().flatten().collect();
    diesel::insert_into(contest_item_descs)
        .values(new_descs)
        .execute(&*db)
        .expect("Error inserting contest item descriptions");

    responses::created().data(json!({
        "message": "successfully created a new contest",
        "id": res_contest.id,
    }))
}
