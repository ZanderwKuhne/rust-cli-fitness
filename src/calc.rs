// Calculations used in the application
use chrono::{Datelike, Local, NaiveDate};

//Calculate the calories of the logged meal
pub fn macros_calories(protein: u32, carbs: u32, fat: u32) -> u32 {
    let calories: u32 = (protein * 4) + (carbs * 4) + (fat * 8);
    calories
}

//Calculate the User Basal Metabolic Rate
pub fn calc_bmr(height: f32, weight: f32, gender: &str, age: u8) -> f32 {
    let bmr: f32;
    if gender == "male" {
        bmr = 66.47 + (13.75 * weight) + (5.003 * height) - (6.755 * age as f32);
    } else {
        bmr = 655.1 + (9.563 * weight) + (1.850 * height) - (4.676 * age as f32);
    }
    bmr
}

// Derive age from birthdate instead of staticly typed number
pub fn get_age(birthdate: NaiveDate) -> u8 {
    let now = Local::now().date_naive();
    let mut age = now.year() - birthdate.year();

    if now.month() < birthdate.month()
        || (now.month() == birthdate.month() && now.day() < birthdate.day())
    {
        age -= 1;
    }
    age as u8
}

// A basic step to calorie converter using user details and MET calculations
pub fn step_to_calories(steps: u32, weight: f32, height: f32) -> u32 {
    let height_m = height / 100.0;
    let stride = height_m * 0.414;
    let distance_km = (steps as f32 * stride) / 1000.0;
    (distance_km * weight * 0.5) as u32
}

pub fn calc_dyna_dri(bmr: f32, act_level: u8, current_w: f32, goal_w: f32) -> f32 {
    let maintenance = match act_level {
        1 => bmr * 1.2,
        2 => bmr * 1.375,
        3 => bmr * 1.55,
        4 => bmr * 1.725,
        5 => bmr * 1.9,
        _ => bmr * 1.2,
    };

    let diff = current_w - goal_w;

    if diff >= 0.5 {
        (maintenance - 500.0).max(1200.0)
    } else if diff <= -0.5 {
        maintenance + 300.0
    } else {
        maintenance
    }
}
