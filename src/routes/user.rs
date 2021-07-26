use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use crate::models::{User, NewUser};
use serde_json::Value;


#[get("/", format="application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let users = User::all(&conn);

    Json(json!({
        "status" : 200,
        "result" : users
    }))
}

#[post("/", format = "application/json", data = "<new_user>")]
pub fn new(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert(new_user.into_inner(), &conn).unwrap(),
    }))
}
