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

//Calculate Recommended Daily Intake using BMR
pub fn calc_dri(bmr: f32, act_level: u8) -> f32 {
    match act_level {
        1 => bmr * 1.2,
        2 => bmr * 1.375,
        3 => bmr * 1.55,
        4 => bmr * 1.725,
        5 => bmr * 1.9,
        _ => 0.0,
    }
}
