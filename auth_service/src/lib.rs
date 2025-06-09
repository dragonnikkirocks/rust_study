
#![allow(unused_imports,dead_code,unused_variables)]

pub mod database;
pub mod user;

pub mod auth_service{

    pub fn authenticate_user(username: String) -> Bool{

        if database::connection::connect().is_ok() {
            println!("User {} authenticated successfully.", username);
            true
        } else {
            println!("Failed to authenticate user {}.", username);
            false
        } 
    }
}




