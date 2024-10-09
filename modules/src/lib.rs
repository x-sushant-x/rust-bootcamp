#[allow(dead_code, unused)]

mod db {
    pub enum Status {
        CONNECTED,
        DISCONNECTED
    }


    pub fn connect_to_database() -> Status {
        Status::CONNECTED
    }
}

mod auth_utils {
    pub struct Credentials {
        username: String,
        password: String
    }
    
    
    pub fn login(creds: Credentials) {
        println!("User Logged In")
    }
}


fn authenticate(creds: auth_utils::Credentials) {
    if let db::Status::CONNECTED = db::connect_to_database() {
        println!("Connected to database");
        auth_utils::login(creds);
    }
}