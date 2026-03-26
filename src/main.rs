mod calc;
mod display;
mod helper;
mod logic;
mod users;

use crate::{helper::get_input, users::User};
use crossterm::{
    execute,
    style::Stylize,
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
                display::center_vertically(5);
                display::print_centered("Welcome!");
                display::print_centered("1. Login (Load User)");
                display::print_centered("2. Create New User");
                display::print_centered("q. Quit");

                let choice = helper::get_string_input("> ");
                match choice.as_str() {
                    "1" => {
                        let name = helper::get_string_input("Enter name: ");
                        match crate::logic::pull_user(&name) {
                            Ok(user) => current_user = Some(user),
                            Err(_) => {
                                display::print_centered("User not found!");
                                helper::pause();
                            }
                        }
                    }
                    "2" => {
                        if logic::store_user().is_ok() {
                            display::print_centered("User created! Please login.");
                        }
                        helper::pause();
                    }
                    "q" => break,
                    _ => {
                        display::print_centered("Invalid option.");
                        helper::pause();
                    }
                }
            }
            Some(user) => {
                helper::clear_screen();
                display::center_vertically(8);
                display::show_user(user);
                display::print_centered("1. Log a Meal");
                display::print_centered("2. Log an Activity");
                display::print_centered("3. View User Dashboard");
                display::print_centered("4. Manage User Details");
                display::print_centered("5. Convert steps to calories");
                display::print_centered("q. Logout");

                let choice = helper::get_string_input("> ");
                match choice.as_str() {
                    "1" => {
                        helper::clear_screen();
                        display::print_centered("Enter meal details:");

                        let protein = helper::get_input("Protein (g): ");
                        let carbs = helper::get_input("Carbs (g): ");
                        let fat = helper::get_input("Fat (g): ");

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
                                display::print_centered("Logged Activity!")
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
                        display::center_vertically(6);
                        display::print_centered("--- Manage Data ---");
                        display::print_centered("1. Update Current Weight");
                        display::print_centered("2. Update Goal Weight");
                        display::print_centered("3. Delete Meal");
                        display::print_centered("4. Delete Activity");
                        display::print_centered("5. Delete Logged Weight");
                        display::print_centered("q. Back");

                        let sub_choice = helper::get_string_input("> ");
                        match sub_choice.as_str() {
                            "1" => {
                                helper::clear_screen();
                                if user.system {
                                    let new_w = helper::get_input("Enter new weight in kg: ");
                                    logic::update_user(&user.name, new_w as f32)
                                        .and_then(|_| logic::pull_user(&user.name))
                                        .map(|updated| {
                                            *user = updated;
                                            println!("Updated User!");
                                        })
                                        .ok();
                                    helper::pause();
                                } else {
                                    let new_w = helper::get_input("Enter new weight in lb: ");
                                    logic::update_user(&user.name, (new_w as f32) / 2.205)
                                        .and_then(|_| logic::pull_user(&user.name))
                                        .map(|updated| {
                                            *user = updated;
                                            println!("Updated User!");
                                        })
                                        .ok();
                                    helper::pause();
                                }
                            }
                            "2" => {
                                helper::clear_screen();
                                let new_w = helper::get_input("Enter new goal weight: ");
                                logic::update_goal_weight(&user.name, new_w as f32)
                                    .and_then(|_| logic::pull_user(&user.name))
                                    .map(|update| {
                                        *user = update;
                                        println!("Updated User!");
                                    })
                                    .ok();
                                helper::pause();
                            }
                            "3" => {
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
                            "4" => {
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
                            "5" => {
                                helper::clear_screen();
                                display::list_weights(user);

                                if !user.weights.is_empty() {
                                    let input = helper::get_input(
                                        "\nEnter the [number] to delete (or 999 to cancel): ",
                                    );
                                    if input != 999 {
                                        if logic::delete_weight_entry(&user.name, input as usize)
                                            .is_ok()
                                        {
                                            if let Ok(updated) = logic::pull_user(&user.name) {
                                                *user = updated;
                                                println!(
                                                    "Weight entry removed and stats recalculated!"
                                                );
                                            }
                                        } else {
                                            println!("Invalid selection.");
                                        }
                                    }
                                }
                                helper::pause();
                            }
                            "q" => continue,
                            _ => {
                                println!("Invalid option.");
                                helper::pause();
                            }
                        }
                    }
                    "5" => {
                        helper::clear_screen();
                        let steps = helper::get_input("Enter stepcount to convert: ");
                        let step_cal = calc::step_to_calories(steps, user.weight, user.height);
                        println!(
                            "{} Steps converts to an about {} kcal burned!",
                            steps.to_string().red(),
                            step_cal.to_string().red()
                        );
                        let log_steps = helper::get_string_input(
                            "\nWould you like to log this as a workout for today?(y/n)",
                        );
                        match log_steps.as_str() {
                            "y" => {
                                let act: String = "Walking".to_string();
                                logic::log_activity(&user.name, step_cal, act)
                                    .and_then(|_| logic::pull_user(&user.name))
                                    .map(|update| {
                                        *user = update;
                                        println!("Activity logged!");
                                    })
                                    .ok();
                                helper::pause();
                            }
                            "n" => continue,
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
