mod calc;
mod display;
mod helper;
mod logic;
mod users;

use crate::{helper::get_input, users::User};
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
                        if logic::store_user().is_ok() {
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
                println!("4. Manage User Details");
                println!("q. Logout");

                let choice = helper::get_string_input("> ");
                match choice.as_str() {
                    "1" => {
                        helper::clear_screen();
                        println!("Enter meal details:");

                        let protein = helper::get_input("Protein (g): ");
                        let carbs = helper::get_input("Carbs (g)");
                        let fat = helper::get_input("Fat (g)");

                        let kcal = calc::macros_calories(protein, carbs, fat);

                        logic::log_meal(&user.name, kcal, protein, fat, carbs)
                            .and_then(|_| logic::pull_user(&user.name))
                            .map(|updated| {
                                *user = updated;
                                println!("Logged {} kcal!", kcal);
                            })
                            .ok();
                        helper::pause();
                    }
                    "2" => {
                        helper::clear_screen();
                        let act_name = helper::get_string_input("Activity name (e.g. Running): ");
                        let burned = helper::get_input("Calories burned: ");

                        logic::log_activity(&user.name, burned, act_name)
                            .and_then(|_| logic::pull_user(&user.name))
                            .map(|updated| {
                                *user = updated;
                                println!("Logged Activity!")
                            })
                            .ok();
                        helper::pause();
                    }
                    "3" => {
                        helper::clear_screen();
                        display::show_dashboard(user);
                        helper::pause();
                    }
                    "4" => {
                        helper::clear_screen();
                        println!("--- Manage Data ---");
                        println!("1. Update Current Weight");
                        println!("2. Delete Meal");
                        println!("3. Delete Activity");
                        println!("q. Back");

                        let sub_choice = helper::get_string_input("> ");
                        match sub_choice.as_str() {
                            "1" => {
                                helper::clear_screen();
                                let new_w = helper::get_input("Enter new weight: ");
                                logic::update_user(&user.name, new_w as f32)
                                    .and_then(|_| logic::pull_user(&user.name))
                                    .map(|updated| {
                                        *user = updated;
                                        println!("Updated User!");
                                    })
                                    .ok();
                                helper::pause();
                            }
                            "2" => {
                                helper::clear_screen();
                                display::list_meals(&user);

                                let index = helper::get_input("Select meal by Index: ");
                                logic::delete_meal(&user.name, index as usize)
                                    .and_then(|_| logic::pull_user(&user.name))
                                    .map(|update| {
                                        *user = update;
                                        println!("Meal removed!");
                                    })
                                    .ok();
                                helper::pause();
                            }
                            "3" => {
                                helper::clear_screen();
                                display::list_activities(&user);

                                let index = get_input("Select Activity by Index: ");
                                logic::delete_activity(&user.name, index as usize)
                                    .and_then(|_| logic::pull_user(&user.name))
                                    .map(|update| {
                                        *user = update;
                                        println!("Activity removed!");
                                    })
                                    .ok();
                                helper::pause();
                            }
                            "q" => break,
                            _ => {
                                println!("Invalid option.");
                                helper::pause();
                            }
                        }
                    }
                    "q" => current_user = None,
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
