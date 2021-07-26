use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use crate::models::{User, NewUser};
use serde_json::Value;



#[post("/", format = "application/json", data = "<to_auth>")]
pub fn authenticate(conn: DbConn, to_auth: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::authenticate(to_auth.into_inner(), &conn).unwrap(),
    }))
}
