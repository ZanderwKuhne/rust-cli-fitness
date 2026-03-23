mod calc;
mod display;
mod logic;
mod users;

use std::io::{self, Write};

fn main() {
    println!("Welcome!");
    loop {
        print!("\nChoose an option: [1] Create User, [2] Show User Info [q] Quit\n> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failer to capture choice");

        match choice.trim() {
            "1" => {
                if let Err(e) = logic::store_user() {
                    eprint!("Error saving user: {}", e);
                }
            }
            "2" => {
                print!("Enter user name to load: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                match logic::pull_user(&name) {
                    Ok(user) => display::show_user(&user),
                    Err(_) => println!("User not found! (Check if {}.json exists)", name.trim()),
                }
            }
            "q" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}
