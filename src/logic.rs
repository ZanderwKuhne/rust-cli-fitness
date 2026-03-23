use std::{fs, io};

use chrono::{DateTime, Local};

use crate::calc::{self, calc_bmr, calc_dri};
use crate::users::User;

//store the user in the json file
pub fn store_user() -> std::io::Result<()> {
    let mut sys: String = String::new();
    let mut u_name: String = String::new();
    let mut u_age: String = String::new();
    let mut u_gender: String = String::new();
    let mut u_height: String = String::new();
    let mut u_weight: String = String::new();
    let mut g_weight: String = String::new();
    let mut activity: String = String::new();

    println!("Enter your name:\n");
    io::stdin()
        .read_line(&mut u_name)
        .expect("Failed to capture name");
    println!("Enter your age:\n");
    io::stdin()
        .read_line(&mut u_age)
        .expect("Failed to capture age");
    let f_age: u8 = u_age.trim().parse().expect("No byte read");
    println!("Metric system: 1\nImperial system: 2\n");
    io::stdin()
        .read_line(&mut sys)
        .expect("Failed to capture system");

    let u_system: bool = if sys.trim() == "1" { true } else { false };

    println!("What is your gender?\n");
    io::stdin()
        .read_line(&mut u_gender)
        .expect("Failed to capture gender");

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
    let u_bmr: f32 = calc_bmr(f_height, f_weight, &u_gender, f_age);
    let u_dri: f32 = calc_dri(u_bmr, u_act_level);

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
        date: Local::now(),
    };

    let json_log = serde_json::to_string_pretty(&user)?;
    let file_name = format!("{}.json", user.name);
    fs::write(file_name, json_log)?;

    println!("User details captured");
    Ok(())
}

//Load user data
pub fn pull_user(name: &str) -> std::io::Result<User> {
    let file_path = format!("{}.json", name.trim());
    let json_data = fs::read_to_string(file_path)?;
    let user: User = serde_json::from_str(&json_data)?;
    Ok(user)
}

//Update stored user information
fn update_user(/* User struct */) /* not sure if it should return a value */ {}

//Log activities for user
fn log_activities() /*not sure if it should return a value, maybe activity struct? */ {}

//Log a meal for user
fn log_meal() /*not sure if it should return a value, maybe meal struct? */ {}
