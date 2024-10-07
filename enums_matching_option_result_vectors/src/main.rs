use serde::Serialize;

#[derive(Debug, Serialize)]
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String
    }
}

impl Command {
    fn serialize(&self) -> String {
        let result = serde_json::to_string(self).unwrap();
        result
    }
}

fn main() {
    let undo = Command::Undo;
    let redo = Command::Redo;

    let add_text = Command::AddText("Hello World".to_string());
    let move_cursor = Command::MoveCursor(0, 0);
    let replace = Command::Replace {
        from: "Hello".to_string(),
        to: "World".to_string()
    };

    println!("{:?}", undo);
    println!("{:?}", redo);
    println!("{:?}", add_text);
    println!("{:?}", move_cursor);
    println!("{:?}", replace);

    let serialized_command = replace.serialize();
    println!("serialized_command: {serialized_command}");

    let username = get_username(1);
    match &username {
        Some(name) => println!("Hi {name}"),
        None => println!("No user found")
    }

    if let Some(name) = username {
        println!("Hi {name}")
    }

    let mut v = Vec::new();
    v.push("Hi");
    v.push("Sushant");
    v.push("Here");

    let mut v2 = vec![1, 2, 3];

    if let Some(el) = v2.get(1) {
        println!("{el}")
    }

    for e in v {
        v2.push(e.parse::<i32>().unwrap());
    }
}


fn get_username(user_id: i32) -> Option<String> {
    // if user_id == 1 {
    //     Some(String::from("Sushant"))
    // } else {
    //     None
    // }

    let query = format!("SELECT username FROM users WHERE id = {user_id}");
    let db_result = query_db(query);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Can not use empty query."))
    } else {
        Ok(String::from("Sushant"))
    }
}