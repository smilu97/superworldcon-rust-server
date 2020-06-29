use rocket::post;
use rocket_contrib::json::Json;
use serde_json::json;
use diesel::prelude::*;

use crate::database::DbConn;
use crate::responses::{self, APIResponse};

use crate::models::user::User;
use crate::models::tournament::{
    Tournament,

    NewTournament,
    NewMatchRecord,

    TournamentInput,
};

#[post("/tournament", data = "<in_tour>", format = "application/json")]
pub fn handle_post_tournament(user: User, db: DbConn, in_tour: Json<TournamentInput>) -> APIResponse {
    use crate::schema::tournaments::dsl::*;
    use crate::schema::match_records::dsl::*;

    let new_tour = NewTournament {
        user_id: user.id,
        contest_id: in_tour.contest_id,
    }; 
    let tour: Tournament = diesel::insert_into(tournaments)
        .values(new_tour)
        .get_result(&*db)
        .expect("Failed to insert new tournament");
    
    let new_records: Vec<NewMatchRecord> = (&in_tour.records).into_iter().map(|x| -> NewMatchRecord {
        NewMatchRecord {
            size: x.size,
            win_id: x.win_id,
            lose_id: x.lose_id,
            tournament_id: tour.id,
        }
    }).collect();
    diesel::insert_into(match_records)
        .values(new_records)
        .execute(&*db)
        .expect("Failed to insert new records");
    
    responses::created().data(json!({
        "message": "Successfully created new tournament",
        "id": tour.id,
    }))
}
