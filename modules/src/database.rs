pub enum Status {
    CONNECTED,
    DISCONNECTED
}


pub fn connect_to_database() -> Status {
    Status::CONNECTED
}