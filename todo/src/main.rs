use std::io;

struct TodoItem {
    name: String,
}

struct TodoList {
    items: Vec<TodoItem>,
}

enum Command {
    Add(String),
    Delete(String),
    Edit(String, String),
    Show,
    End,
}

impl TodoList {
    fn add_item(&mut self, name: String) {
        let item = TodoItem { name };
        self.items.push(item);
    }

    fn delete_item(&mut self, name: String) {
        self.items.retain(|item| item.name != name);
    }

    fn edit_item(&mut self, name: String, new_name: String) {
        for item in &mut self.items {
            if item.name == name {
                item.name = new_name.clone();
                break;
            }
        }
    }

    fn show_items(&self) {
        for (i, item) in self.items.iter().enumerate() {
            println!("{}: {}", i + 1, item.name);
        }
    }
}

fn parse_command(input: &str) -> Option<Command> {
    let mut parts: std::str::SplitN<'_, char> = input.trim().splitn(3, ' ');
    let command: &str = parts.next()?;
    let name: &str = parts.next().unwrap_or("");
    let new_name: &str = parts.next().unwrap_or("");

    match command {
        "add" => Some(Command::Add(name.to_string())),
        "delete" => Some(Command::Delete(name.to_string())),
        "edit" => Some(Command::Edit(name.to_string(), new_name.to_string())),
        "show" => Some(Command::Show),
        "end" => Some(Command::End),
        _ => None,
    }
}

fn main() {
    let mut todo_list = TodoList { items: vec![] };

    loop {
        let mut input = String::new();
        println!("Enter a command: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(command) = parse_command(&input) {
                    match command {
                        Command::Add(name) => todo_list.add_item(name),
                        Command::Delete(name) => todo_list.delete_item(name),
                        Command::Edit(name, new_name) => todo_list.edit_item(name, new_name),
                        Command::Show => todo_list.show_items(),
                        Command::End => return,
                    }
                } else {
                    println!("Invalid command.");
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                return;
            }
        }
    }
}
