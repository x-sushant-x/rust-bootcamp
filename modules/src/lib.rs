#[allow(dead_code, unused)]

mod database;
mod auth_utils;

    


use crate::auth_utils::login;
use crate::database::connect_to_database;
use crate::database::Status;

fn authenticate(creds: auth_utils::Credentials) {
    if let Status::CONNECTED = connect_to_database() {
        println!("Connected to database");
        login(creds);
    }
}