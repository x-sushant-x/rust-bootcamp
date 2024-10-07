#[derive(Debug)]
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
}
