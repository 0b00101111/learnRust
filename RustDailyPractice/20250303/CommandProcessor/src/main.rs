enum Command {
    InsertText(String),
    DeleteText(String),
    MoveCursor(i32, i32),
    Save(Option<String>),
    Quit,
}

fn execute_command(command: Command) {
    match command {
        Command::InsertText(text) => {
            println!("{}", text);
        },
        Command::DeleteText(text) => {
            println!("{}", text);
        }
        Command::MoveCursor(x, y) => {
            println!("Cursor moved to ({}, {})", x, y);
        },
        Command::Save(text) => {
            println!("Saved");
        },
        Command::Quit => {
            std::process::exit(0);
        },
    }
}


fn main() {
    let commands = vec![
        String::from("Insert this"),
        String::from("Delete this"),
        String::from("Move cursor"),
        String::from("Save"),
        String::from("Quit"),
    ];

    for (i, command) in commands.iter().enumerate() {
        execute_command(Command::InsertText(command.to_string()));
    }
}