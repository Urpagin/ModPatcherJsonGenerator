//! This small utility is to facilitate the creation/addition of items in the
//! json list for the ModsUpgrader program.

use std::io::Write;

use crate::terminal_actions::{
    AddItemCommand,
    DeleteItemCommand,
    ModifyItemCommand,
    QuitCommand,
    ShowItemsCommand,
    TerminalAction,
    TerminalCommand,
};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Action {
    Add,
    Delete,
    Update,
}

impl Action {
    fn to_str(&self) -> &str {
        match self {
            Action::Add => "ADD",
            Action::Delete => "DELETE",
            Action::Update => "UPDATE",
        }
    }
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("ADD") {
            Ok(Action::Add)
        } else if value.eq_ignore_ascii_case("DELETE") {
            Ok(Action::Delete)
        } else if value.eq_ignore_ascii_case("UPDATE") {
            Ok(Action::Update)
        } else {
            Err(format!("Invalid action: {}", value))
        }
    }
}


/// This struct models an item in the json.
/// An `Item` has:
///
/// `filename`: an `Item` is first of all a file, hence it has a `filename` property for the client to save.
///
/// `action`: an enum `Action` that variants model the action that the client will do with the `Item`.
///
/// `download_link`: the direct link to the file.
#[derive(Debug, Clone)]
struct Item {
    filename: String,
    action: Action,
    download_link: String,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            filename: String::default(),
            action: Action::Add,
            download_link: String::default(),
        }
    }
}

/// This function displays a `message` to stdin using `print!` and returns the user input
/// into an owned `String`.
fn read_user_input(message: &str) -> String {
    print!("{}", message);
    std::io::stdout().flush().expect("ERROR: could not flush stdout!");

    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().to_string(),
        Err(_) => String::new(),
    }
}

fn read_user_item() -> Item {
    println!("\n--- Adding a new item ---\n");
    let mut item = Item::default();

    // Get filename
    loop {
        let user_input = read_user_input("Filename: ");
        if user_input.is_empty() {
            println!("WARNING: filename is empty, please try again.");
            continue;
        }
        item.filename = user_input;
        break;
    }

    println!("\n");

    // Get action
    loop {
        let user_input = read_user_input("Action (ADD, DELETE, UPDATE): ").to_uppercase();
        if Action::Add.to_str() == user_input {
            item.action = Action::Add;
            break;
        } else if Action::Update.to_str() == user_input {
            item.action = Action::Update;
            break;
        } else if Action::Delete.to_str() == user_input {
            item.action = Action::Delete;
            break;
        } else {
            println!("{}", INVALID_INPUT_MSG);
            continue;
        }
    }

    println!("\n");

    // Get download link
    loop {
        let user_input = read_user_input("Direct download link: ");
        if user_input.is_empty() {
            println!("WARNING: filename is empty, please try again.");
            continue;
        }
        item.download_link = user_input;
        break;
    }

    item
}

mod terminal_actions {
    use crate::{Action, INVALID_INPUT_MSG, Item, read_user_input, read_user_item};

    fn print_enumerated_items(items: &[Item]) {
        println!("\n\n-- ITEMS LISTED --\n\n");
        items.iter().enumerate().for_each(|(idx, item)| {
            // idx + 1 = indexing from 1, for human readability.
            println!("{}: {:?}", idx + 1, item);
        });
    }

    // Read from stdin for user input.
    fn read_item_idx(items: &[Item]) -> usize {
        loop {
            print_enumerated_items(items);
            let user_input = read_user_input("Enter the item number you wish to change: ");
            match user_input.parse::<usize>() {
                Ok(idx) => {
                    match idx.checked_sub(1) {
                        Some(subbed_idx) if subbed_idx < items.len() => return subbed_idx,
                        _ => {
                            println!("WARNING: index out of bounds, please try again.");
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("{}", INVALID_INPUT_MSG);
                }
            }
        }
    }

    pub trait TerminalCommand {
        fn execute(&mut self);
    }

    pub enum TerminalAction {
        Quit,
        AddItem,
        ModifyItem,
        DeleteItem,
        Show,
    }

    fn print_items_json(items: &[Item]) {
        let json_items: Vec<serde_json::Value> = items.iter().map(|item| {
            serde_json::json!({
                "mod_filename": item.filename,
                "action": item.action.to_str(),
                "download_link": item.download_link,
        })
        }).collect();

        let json_output = serde_json::to_string_pretty(&json_items).unwrap();
        println!("\n\n{}", json_output);
    }


    // Quit (quits and shows the final json)
    pub struct QuitCommand<'a> {
        pub items: &'a Vec<Item>,
    }

    impl<'a> TerminalCommand for QuitCommand<'a> {
        fn execute(&mut self) {
            if !self.items.is_empty() {
                print_items_json(self.items);
            }
            std::process::exit(0);
        }
    }


    // Add Item
    pub struct AddItemCommand<'a> {
        pub(crate) items: &'a mut Vec<Item>,
    }

    impl<'a> TerminalCommand for AddItemCommand<'a> {
        fn execute(&mut self) {
            let item = read_user_item();
            self.items.push(item);
        }
    }


    // Modify Item
    pub struct ModifyItemCommand<'a> {
        pub(crate) items: &'a mut Vec<Item>,
    }

    impl<'a> ModifyItemCommand<'a> {
        fn read_user() -> String {
            println!("What do you want to modify?");
            loop {
                let user_input = read_user_input("1: Filename\n2: Action\n3: Direct download link\n-> ");
                match user_input.as_str() {
                    "1" | "2" | "3" => return user_input,
                    _ => println!("{}", INVALID_INPUT_MSG),
                }
            }
        }

        fn modify_item(item: &mut Item) {
            match Self::read_user().as_str() {
                "1" => { // filename
                    loop {
                        let user_input = read_user_input("Filename new value: ");
                        if user_input.is_empty() {
                            println!("{}", INVALID_INPUT_MSG);
                            continue;
                        }
                        item.filename = user_input;
                        return;
                    }
                }
                "2" => { // action
                    loop {
                        let user_input = read_user_input("Action new value (ADD, DELETE, UPDATE) : ");
                        match Action::try_from(user_input.as_str()) {
                            Ok(action) => {
                                item.action = action;
                                return;
                            }
                            Err(_) => {
                                println!("{}", INVALID_INPUT_MSG);
                                continue;
                            }
                        }
                    }
                }
                "3" => { // download link
                    loop {
                        let user_input = read_user_input("Direct download link new value: ");
                        if user_input.is_empty() {
                            println!("{}", INVALID_INPUT_MSG);
                            continue;
                        }
                        item.download_link = user_input;
                        return;
                    }
                }
                _ => {}
            }
        }
    }

    impl<'a> TerminalCommand for ModifyItemCommand<'a> {
        fn execute(&mut self) {
            if self.items.is_empty() {
                println!("WARNING: no items, cannot modify.");
                return;
            }
            let user_item_idx = read_item_idx(self.items);
            Self::modify_item(&mut self.items[user_item_idx]);
        }
    }


    // Delete Item
    pub struct DeleteItemCommand<'a> {
        pub items: &'a mut Vec<Item>,
    }

    impl<'a> TerminalCommand for DeleteItemCommand<'a> {
        fn execute(&mut self) {
            if self.items.is_empty() {
                println!("WARNING: no items, cannot delete.");
                return;
            }
            let user_item_idx = read_item_idx(self.items);
            self.items.remove(user_item_idx);
        }
    }

    // Show

    pub struct ShowItemsCommand<'a> {
        pub(crate) items: &'a Vec<Item>,
    }

    impl<'a> TerminalCommand for ShowItemsCommand<'a> {
        fn execute(&mut self) {
            if self.items.is_empty() {
                println!("WARNING: no items, cannot show.");
                return;
            }
            print_items_json(self.items);
        }
    }
}

fn get_terminal_action() -> TerminalAction {
    loop {
        let user_input = read_user_input("Enter command (q: Quit, a: Add, m: Modify, d: Delete, s: Show): ");
        match user_input.to_lowercase().as_str() {
            "q" => return TerminalAction::Quit,
            "a" => return TerminalAction::AddItem,
            "m" => return TerminalAction::ModifyItem,
            "d" => return TerminalAction::DeleteItem,
            "s" => return TerminalAction::Show,
            _ => println!("{}", INVALID_INPUT_MSG)
        }
    }
}

fn execute_command(term_action: TerminalAction, items: &mut Vec<Item>) {
    match term_action {
        TerminalAction::Quit => QuitCommand { items }.execute(),
        TerminalAction::AddItem => AddItemCommand { items }.execute(),
        TerminalAction::ModifyItem => ModifyItemCommand { items }.execute(),
        TerminalAction::DeleteItem => DeleteItemCommand { items }.execute(),
        TerminalAction::Show => ShowItemsCommand { items }.execute(),
    }
}


const INVALID_INPUT_MSG: &str = "WARNING: Invalid input, please try again.";

fn main() {
    let mut items: Vec<Item> = Vec::new();
    loop {
        let action = get_terminal_action();
        execute_command(action, &mut items);
    }
}
