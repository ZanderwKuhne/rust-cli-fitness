use colored::Colorize;
use std::{fs, io};

use chrono::{Duration, Local, NaiveDate};

use crate::calc;
use crate::display;
use crate::helper;
use crate::helper::get_input;
use crate::users::{LogActivity, LogMeal, User};

//store the user in the json file
pub fn store_user() -> std::io::Result<()> {
    std::fs::create_dir_all("data")?;
    let mut sys: String = String::new();
    let mut u_name: String = String::new();
    let mut u_birth: String = String::new();
    let mut u_gender: String = String::new();
    let mut u_height: String = String::new();
    let mut u_weight: String = String::new();
    let mut g_weight: String = String::new();
    let mut activity: String = String::new();

    println!("Enter your name:\n");
    io::stdin()
        .read_line(&mut u_name)
        .expect("Failed to capture name");
    println!("Enter your birthdate (YYYY-MM-DD):\n");
    io::stdin()
        .read_line(&mut u_birth)
        .expect("Failed to capture age");
    let u_birthdate = match NaiveDate::parse_from_str(&u_birth.trim(), "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            println!("Invalid format! Defaulting to 1990-01-01.");
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()
        }
    };
    let f_age = calc::get_age(u_birthdate);
    loop {
        sys.clear();
        println!("Metric system: 1\nImperial system: 2\n");
        io::stdin()
            .read_line(&mut sys)
            .expect("Failed to capture system");
        match sys.trim() {
            "1" => break,
            "2" => break,
            _ => {
                println!("Invalid option: {}", sys);
                continue;
            }
        }
    }

    let u_system: bool = if sys.trim() == "1" { true } else { false };

    loop {
        u_gender.clear();
        println!("What is your gender? (male/female)\n");
        io::stdin()
            .read_line(&mut u_gender)
            .expect("Failed to capture gender");
        match u_gender.trim().to_lowercase().as_str() {
            "male" => break,
            "female" => break,
            _ => {
                println!("Invalid option: {}", u_gender);
                continue;
            }
        }
    }

    if u_system {
        println!("What is your height in cm?\n");
        io::stdin()
            .read_line(&mut u_height)
            .expect("Failed to capture height");
        println!("What is your weight in kg?\n");
        io::stdin()
            .read_line(&mut u_weight)
            .expect("Failed to capture weight");
        println!("What is your goal weight in kg?\n");
        io::stdin()
            .read_line(&mut g_weight)
            .expect("Failed to capture goal weight");
    } else {
        println!("What is your height in inches?\n");
        io::stdin()
            .read_line(&mut u_height)
            .expect("Failed to capture height");
        println!("What is your weight in lb?\n");
        io::stdin()
            .read_line(&mut u_weight)
            .expect("Failed to capture weight");
        println!("What is your goal_weight in lb?\n");
        io::stdin()
            .read_line(&mut g_weight)
            .expect("Failed to capture goal weight");
    }
    let (f_height, f_weight, f_goal_weight): (f32, f32, f32) = if u_system {
        (
            u_height.trim().parse().expect("Not a number"),
            u_weight.trim().parse().expect("Not a number"),
            g_weight.trim().parse().expect("Not a number"),
        )
    } else {
        (
            u_height.trim().parse::<f32>().expect("Not a number") * 2.54,
            u_weight.trim().parse::<f32>().expect("Not a number") * 0.453,
            g_weight.trim().parse::<f32>().expect("Not a number") * 0.453,
        )
    };

    println!(
        "What is your activity level?\n1 - Sedentary\n2 - Lightly Active\n3 - Moderately Active\n4 - Very Active\n5 - Extremely Active"
    );
    io::stdin()
        .read_line(&mut activity)
        .expect("Failed to capture activity level");

    let u_act_level: u8 = activity.trim().parse().expect("No byte read");
    let u_bmr: f32 = calc::calc_bmr(f_height, f_weight, &u_gender, f_age);
    let u_dri: f32 = calc::calc_dyna_dri(u_bmr, u_act_level, f_weight, f_goal_weight);

    let user = User {
        name: u_name.trim().to_string(),
        gender: u_gender.trim().to_string(),
        weight: f_weight,
        height: f_height,
        goal_weight: f_goal_weight,
        system: u_system,
        act_level: u_act_level,
        age: f_age,
        bmr: u_bmr,
        dri: u_dri,
        birthdate: u_birthdate,
        weights: Vec::new(),
        date: Local::now(),
        meals: Vec::new(),
        activities: Vec::new(),
    };

    let json_log = serde_json::to_string_pretty(&user)?;
    let file_name = format!("data/{}.json", user.name.trim());
    fs::write(file_name, json_log)?;

    println!("User details captured");
    Ok(())
}

//Load user data
pub fn pull_user(name: &str) -> std::io::Result<User> {
    let file_path = format!("data/{}.json", name.trim());
    let json_data = fs::read_to_string(file_path)?;
    let user: User = serde_json::from_str(&json_data)?;
    Ok(user)
}

//Update stored user information
pub fn update_user(name: &str, new_weight: f32) -> std::io::Result<()> {
    let mut user = pull_user(name)?;
    user.weights.push((Local::now().date_naive(), new_weight));
    user.weight = new_weight;

    user.bmr = crate::calc::calc_bmr(user.height, user.weight, &user.gender, user.age);
    user.dri = crate::calc::calc_dyna_dri(user.bmr, user.act_level, user.weight, user.goal_weight);

    let json = serde_json::to_string_pretty(&user)?;
    fs::write(format!("data/{}.json", user.name.trim()), json)?;
    Ok(())
}

pub fn update_goal_weight(name: &str, new_weight: f32) -> std::io::Result<()> {
    let mut user = pull_user(name)?;
    user.goal_weight = new_weight;

    let json = serde_json::to_string_pretty(&user)?;
    fs::write(format!("data/{}.json", user.name.trim()), json)?;
    Ok(())
}

pub fn delete_meal(name: &str, index: usize) -> std::io::Result<()> {
    let mut user = pull_user(name)?;
    if index < user.meals.len() {
        user.meals.remove(index);
        let json = serde_json::to_string_pretty(&user)?;
        fs::write(format!("data/{}.json", user.name.trim()), json)?;
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Index out of range",
        ))
    }
}

pub fn delete_activity(name: &str, index: usize) -> std::io::Result<()> {
    let mut user = pull_user(name)?;
    if index < user.activities.len() {
        user.activities.remove(index);
        let json = serde_json::to_string_pretty(&user)?;
        fs::write(format!("{}.json", user.name.trim()), json)?;
    }
    Ok(())
}

//Log activities for user
pub fn log_activity(name: &str, kcal_burnt: u32, act_type: String) -> std::io::Result<()> {
    let mut user = pull_user(name)?;

    let new_activity = LogActivity {
        act_type,
        kcal_burn: kcal_burnt,
        date: Local::now(),
    };
    user.activities.push(new_activity);

    let json = serde_json::to_string_pretty(&user)?;
    fs::write(format!("data/{}.json", name.trim()), json)?;

    Ok(())
}

//Log a meal for user
pub fn log_meal(name: &str, kcal: u32, protein: u32, fat: u32, carbs: u32) -> std::io::Result<()> {
    let mut user: User = pull_user(&name)?;

    let new_meal = LogMeal {
        kcal,
        protein,
        carbs,
        fat,
        date: Local::now(),
    };

    user.meals.push(new_meal);
    let json = serde_json::to_string_pretty(&user)?;
    fs::write(format!("data/{}.json", name.trim()), json)?;
    Ok(())
}

// Calculate calories for a period of time
pub fn get_calorie_sum(user: &User, days: i64) -> u32 {
    let now = Local::now().date_naive();
    let start_date = now - Duration::days(days);

    user.meals
        .iter()
        .filter(|meal| {
            let meal_date = meal.date.date_naive();
            meal_date >= start_date && meal_date <= now
        })
        .map(|meal| meal.kcal)
        .sum()
}

// Calculate calories burned over a period (Only using current day calories,
// more functionality to be added)
pub fn get_activity_sum(user: &User, days: i64) -> u32 {
    let now = Local::now().date_naive();
    let start_date = now - Duration::days(days);

    user.activities
        .iter()
        .filter(|act| {
            let act_date = act.date.date_naive();
            act_date >= start_date && act_date <= now
        })
        .map(|act| act.kcal_burn)
        .sum()
}
pub fn delete_weight_entry(name: &str, index: usize) -> std::io::Result<()> {
    let mut user = pull_user(name)?;

    if index < user.weights.len() {
        user.weights.remove(index);

        if let Some((_, last_w)) = user.weights.last() {
            user.weight = *last_w;
        }

        user.bmr = crate::calc::calc_bmr(user.height, user.weight, &user.gender, user.age);
        user.dri =
            crate::calc::calc_dyna_dri(user.bmr, user.act_level, user.weight, user.goal_weight);

        let json = serde_json::to_string_pretty(&user)?;
        fs::write(format!("data/{}.json", user.name.trim()), json)?;
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Index out of range!",
        ))
    }
}

pub fn landing_page() {
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
                        match pull_user(&name) {
                            Ok(user) => current_user = Some(user),
                            Err(_) => {
                                display::print_centered("User not found!");
                                helper::pause();
                            }
                        }
                    }
                    "2" => {
                        if store_user().is_ok() {
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
                logged_in(user);
                current_user = None;
            }
        }
    }
}

fn logged_in(user: &mut User) {
    loop {
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
            "1" => log_meal_choice(user),
            "2" => log_act_choice(user),
            "3" => view_dash_choice(user),
            "4" => manage_user_choice(user),
            "5" => calc_steps_choice(user),
            "q" => break,
            _ => {
                println!("Invalid option.");
                helper::pause();
            }
        }
    }
}

fn log_meal_choice(user: &mut User) {
    helper::clear_screen();
    let protein = get_input("Enter Protein in (g): ");
    let carbs = get_input("Enter Carbs in (g): ");
    let fat = get_input("Enter Fat in (g)");
    let kcal = calc::macros_calories(protein, carbs, fat);

    log_meal(&user.name, kcal, protein, fat, carbs)
        .and_then(|_| pull_user(&user.name))
        .map(|update| {
            *user = update;
            display::print_centered("Logged Meal!");
        })
        .ok();
    helper::pause();
}

fn log_act_choice(user: &mut User) {
    helper::clear_screen();
    let act_name = helper::get_string_input("Activity name (e.g. Running): ");
    let burned = helper::get_input("Calories burned: ");

    log_activity(&user.name, burned, act_name)
        .and_then(|_| pull_user(&user.name))
        .map(|updated| {
            *user = updated;
            display::print_centered("Logged Activity!")
        })
        .ok();
    helper::pause();
}

fn view_dash_choice(user: &mut User) {
    helper::clear_screen();
    display::show_dashboard(user);
    helper::pause();
}

fn manage_user_choice(user: &mut User) {
    helper::clear_screen();
    display::center_vertically(6);
    display::print_centered("--- Manage Data ---");
    display::print_centered("1. Update Current Weight");
    display::print_centered("2. Update Goal Weight");
    display::print_centered("3. Delete Meal");
    display::print_centered("4. Delete Activity");
    display::print_centered("5. Delete Logged Weight");
    display::print_centered("q. Back");

    let choice = helper::get_string_input("> ");
    match choice.as_str() {
        "1" => update_weight_choice(user),
        "2" => update_gw_choice(user),
        "3" => delete_meal_choice(user),
        "4" => delete_act_choice(user),
        "5" => delete_logmeal_choice(user),
        "q" => return,
        _ => {
            println!("Invalid option.");
            helper::pause();
        }
    }
}

fn update_weight_choice(user: &mut User) {
    helper::clear_screen();
    if user.system {
        let new_w = helper::get_input("Enter new weight in kg: ");
        update_user(&user.name, new_w as f32)
            .and_then(|_| pull_user(&user.name))
            .map(|updated| {
                *user = updated;
                println!("Updated User!");
            })
            .ok();
        helper::pause();
    } else {
        let new_w = helper::get_input("Enter new weight in lb: ");
        update_user(&user.name, (new_w as f32) / 2.205)
            .and_then(|_| pull_user(&user.name))
            .map(|updated| {
                *user = updated;
                println!("Updated User!");
            })
            .ok();
        helper::pause();
    }
}

fn update_gw_choice(user: &mut User) {
    helper::clear_screen();
    let new_w = helper::get_input("Enter new goal weight: ");
    update_goal_weight(&user.name, new_w as f32)
        .and_then(|_| pull_user(&user.name))
        .map(|update| {
            *user = update;
            println!("Updated User!");
        })
        .ok();
    helper::pause();
}

fn delete_meal_choice(user: &mut User) {
    helper::clear_screen();
    loop {
        display::list_meals(&user);
        let mut input = String::new();
        display::print_centered("Enter Index to delete or q to return: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to capture input");
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("q") {
            break;
        }
        match trimmed.parse::<usize>() {
            Ok(index) => {
                delete_meal(&user.name, index)
                    .and_then(|_| pull_user(&user.name))
                    .map(|update| {
                        *user = update;
                        println!("Meal removed!");
                    })
                    .ok();
                helper::pause();
                helper::clear_screen();
                continue;
            }
            Err(_) => {
                println!("Invalid option.")
            }
        }
    }
}

fn delete_act_choice(user: &mut User) {
    helper::clear_screen();
    display::list_activities(&user);

    let index = helper::get_input("Select Activity by Index: ");
    delete_activity(&user.name, index as usize)
        .and_then(|_| pull_user(&user.name))
        .map(|update| {
            *user = update;
            println!("Activity removed!");
        })
        .ok();
    helper::pause();
}

fn delete_logmeal_choice(user: &mut User) {
    helper::clear_screen();
    display::list_weights(user);

    if !user.weights.is_empty() {
        let input = helper::get_input("\nEnter the [number] to delete (or 999 to cancel): ");
        if input != 999 {
            if delete_weight_entry(&user.name, input as usize).is_ok() {
                if let Ok(update) = pull_user(&user.name) {
                    *user = update;
                    println!("Weight entry removed and stats recalculated!");
                }
            } else {
                println!("Invalid selection.");
            }
        }
    }
    helper::pause();
}

fn calc_steps_choice(user: &mut User) {
    helper::clear_screen();
    let steps = helper::get_input("Enter stepcount to convert: ");
    let step_cal = calc::step_to_calories(steps, user.weight, user.height);
    println!(
        "{} Steps converts to an about {} kcal burned!",
        steps.to_string().red(),
        step_cal.to_string().red()
    );
    let log_steps =
        helper::get_string_input("\nWould you like to log this as a workout for today?(y/n)");
    match log_steps.as_str() {
        "y" => {
            let act: String = "Walking".to_string();
            log_activity(&user.name, step_cal, act)
                .and_then(|_| pull_user(&user.name))
                .map(|update| {
                    *user = update;
                    println!("Activity logged!");
                })
                .ok();
            helper::pause();
        }
        "n" => return,
        _ => {
            println!("Invalid option.");
            helper::pause();
        }
    }
}
