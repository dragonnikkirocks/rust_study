pub mod user_login;

pub mod user_registration;

pub struct User{
    pub id:u32,
    username: String,
    email: String,
    password: String,};


pub enum UserRole {
    Admin,
    Regular,
}