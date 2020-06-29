use rocket::get;
use rocket::http::RawStr;
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
