use std::{fs, io};

use chrono::{Duration, Local, NaiveDate};

use crate::calc::{calc_bmr, calc_dyna_dri, get_age};
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
    let f_age = get_age(u_birthdate);
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
    let u_bmr: f32 = calc_bmr(f_height, f_weight, &u_gender, f_age);
    let u_dri: f32 = calc_dyna_dri(u_bmr, u_act_level, f_weight, f_goal_weight);

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
