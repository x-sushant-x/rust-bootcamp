pub mod models;

pub fn login(creds: models::Credentials) {
    println!("User Logged In {:?}", creds)
}