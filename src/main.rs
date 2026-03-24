mod calc;
mod display;
mod helper;
mod logic;
mod users;

use crate::users::User;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

fn main() {
    execute!(stdout(), EnterAlternateScreen).expect("Failed to enter alternate screen");
    let mut current_user: Option<User> = None;
    loop {
        match &mut current_user {
            None => {
                helper::clear_screen();
                println!("Welcome!");
                println!("1. Login (Load User)");
                println!("2. Create New User");
                println!("q. Quit");

                let choice = helper::get_string_input("> ");
                match choice.as_str() {
                    "1" => {
                        let name = helper::get_string_input("Enter name: ");
                        match crate::logic::pull_user(&name) {
                            Ok(user) => current_user = Some(user),
                            Err(_) => {
                                println!("User not found!");
                                helper::pause();
                            }
                        }
                    }
                    "2" => {
                        if let Ok(_) = logic::store_user() {
                            println!("User created! Please login.");
                        }
                        helper::pause();
                    }
                    "q" => break,
                    _ => {
                        println!("Invalid option.");
                        helper::pause();
                    }
                }
            }
            Some(user) => {
                helper::clear_screen();
                display::show_user(user);
                println!("1. Log a Meal");
                println!("2. Log an Activity");
                println!("3. View User Dashboard");
                println!("4. Logout");

                let choice = helper::get_string_input("> ");
                match choice.as_str() {
                    "1" => {
                        helper::clear_screen();
                        println!("Enter meal details:");

                        let protein = helper::get_input("Protein (g): ");
                        let carbs = helper::get_input("Carbs (g)");
                        let fat = helper::get_input("Fat (g)");

                        let kcal = calc::macros_calories(protein, carbs, fat);

                        if let Ok(_) = logic::log_meal(&user.name, kcal, protein, fat, carbs) {
                            if let Ok(updated) = logic::pull_user(&user.name) {
                                *user = updated;
                            }
                        }
                        helper::pause();
                    }
                    "2" => {
                        helper::clear_screen();
                        let act_name = helper::get_string_input("Activity name (e.g. Running): ");
                        let burned = helper::get_input("Calories burned: ");

                        if let Ok(_) = logic::log_activity(&user.name, burned, act_name) {
                            if let Ok(updated) = logic::pull_user(&user.name) {
                                *user = updated;
                            }
                            println!("Activity logged!");
                            helper::pause();
                        }
                    }
                    "3" => {
                        helper::clear_screen();
                        display::show_dashboard(user);
                        helper::pause();
                    }
                    "4" => current_user = None,
                    _ => {
                        println!("Invalid option.");
                        helper::pause();
                    }
                }
            }
        }
    }
    execute!(stdout(), LeaveAlternateScreen).expect("Failed to leave alternate screen");
}
