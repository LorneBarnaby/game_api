#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]


#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


extern crate hex;
extern crate jsonwebtoken;


use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;


mod db; 
mod schema;
mod models;
mod routes;

use routes::user::*; 
use routes::auth::*; 

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/users",
            routes![new, index],
        )
        .mount(
            "/api/v1/auth",
            routes![authenticate],
        )

}

fn main() {
    rocket().launch();
}

