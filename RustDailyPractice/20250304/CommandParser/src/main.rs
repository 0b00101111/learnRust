// Create a command parser for a simple text editor.

enum Command {
    Quit,
    Save(String),  // Filename
    Move(i32, i32), // x, y coordinates
    Delete(usize),  // Number of characters
    Insert(char),
}

fn parse_command(s: &str) -> Option<Command> {
    let parts: Vec<&str> = s.trim().split_whitespace().collect();

    match parts.get(0)?.to_lowercase().as_str() {
        "q" | "quit" => Some(Command::Quit),
        "s" | "save" => {
            let filename = parts.get(1)?;
            Some(Command::Save(filename.to_string()))
        },
        "m" | "move" => {
            let x = parts.get(1)?.parse::<i32>().ok()?;
            let y = parts.get(2)?.parse::<i32>().ok()?;
            Some(Command::Move(x, y))
        },
        "d" | "delete" => {
            let count = parts.get(1)?.parse::<usize>().ok()?;
            Some(Command::Delete(count))
        },
        "i" | "insert" => {
            let c = parts.get(1)?.chars().next()?;
            Some(Command::Insert(c))
        },
        _ => None,
    }
}

fn main() {
    let commands = vec![
        "quit",
        "save document.txt",
        "move 10 20",
        "delete 5",
        "insert a",
        "invalid",
    ];

    for cmd_str in commands {
        // TODO: Use if let to handle the case where parse_command returns Some(cmd)
        // - For Command::Quit, print "Exiting program"
        // - For Command::Save(filename), print "Saving to {filename}"
        // - For Command::Move(x, y), print "Moving cursor to ({x}, {y})"
        // - For Command::Delete(count), print "Deleting {count} characters"
        // - For Command::Insert(c), print "Inserting character {c}"
        // If parse_command returns None, print "Unknown command"

        if let Some(command) = parse_command(cmd_str) {
            // no need for if let syntax. a single match statement would be more idiomatic. this is
            // just for purpose of practicing if let syntax.
            match command {
                Command::Quit => {
                    println!("Exiting program");
                },
                Command::Save(filename) => {
                    println!("Saving to {}", filename);
                },
                Command::Move(x, y) => {
                    println!("Moving cursor to ({}, {})", x, y);
                },
                Command::Delete(count) => {
                    println!("Deleting {} characters", count);
                },
                Command::Insert(c) => {
                    println!("Inserting character {}", c);
                },
            }
        } else {
            println!("Unknown command");
        }
    }
}
