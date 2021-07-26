use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

use hex::FromHex;


use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};




#[derive(Serialize, Queryable, Debug, Clone)]
pub struct User {
    pub id: i32, 
    pub username : String,
    pub password : String, 
    pub salt: String,
}

#[derive(Serialize, Deserialize, Insertable,Debug,  Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String, 
}

#[derive(Serialize, Deserialize, Insertable,Debug,  Clone)]
#[table_name = "users"]
pub struct NewUserEncrypted {
    pub username: String,
    pub password: String, 
    pub salt: String,
}


fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}

#[derive(Serialize, Deserialize,Debug,  Clone)]
pub struct UserDetails {
    id: i32,
    username: String,
}


impl User {
    pub fn all(conn: &PgConnection) -> Vec<User>{
        all_users 
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("Error loading users")
    }

    pub fn authenticate(user: NewUser, conn: &PgConnection) -> Result<String, Unspecified> {
        let this_user = all_users
            .filter(users::username.eq(user.username))
            .load::<User>(conn)
            .expect("crap");

        let first_user = first(&this_user).unwrap() ; 
        let hash = &first_user.password; 
        let salt = &first_user.salt; 
        let password = user.password; 


        let n_iter = NonZeroU32::new(100_000).unwrap();

        let salt_arr = <[u8;64]>::from_hex(salt).expect("Decoding failed"); 
        let pass_arr = <[u8; 64]>::from_hex(hash).expect("Decoding failed"); 


        let user_verified = pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt_arr,
            password.as_bytes(),
            &pass_arr,
        );


        if user_verified.is_ok() {
            
            let user_details = UserDetails {
                username: first_user.username.to_string(), 
                id: first_user.id,
            };
            match encode(&Header::new(Algorithm::RS256), &user_details, &EncodingKey::from_rsa_pem(include_bytes!("../jwtRS256.key")).unwrap()){
                Ok(token) => Ok(token),
                Err(_) => Ok(String::from("failed to construct token"))
            }

        } else {
            Ok(String::from("not verified"))
        }

        
    }

    pub fn insert(user: NewUser, conn: &PgConnection) -> Result<bool, Unspecified>{

        const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
        let n_iter = NonZeroU32::new(100_000).unwrap();
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; CREDENTIAL_LEN];
        rng.fill(&mut salt)?;

        let password = user.password;
        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );

        let mut encrypted_user = NewUserEncrypted {
            password: HEXUPPER.encode(&pbkdf2_hash),
            username: user.username,
            salt : HEXUPPER.encode(&salt)
        };


        let worked = diesel::insert_into(users::table)
            .values(&encrypted_user)
            .execute(conn)
            .is_ok(); 

        Ok(worked)
    }
}