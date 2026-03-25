use std::usize;

// Module handling display logic
use crate::users::User;
use colored::*;

pub fn show_user(user: &User) {
    println!("\n=== Profile for {} ===", user.name.red());
    println!(
        "Age:    {} | Gender: {}",
        user.age.to_string().red(),
        user.gender.red()
    );
    println!(
        "Weight: {} kg | Height: {} cm",
        format!("{:.1}", user.weight).red(),
        format!("{:.1}", user.height).red()
    );
    println!("----------------------------");
    println!("BMR:    {} kcal", format!("{:2}", user.bmr).red());
    println!(
        "Daily Target (DRI): {} kcal",
        format!("{:.2}", user.dri).red()
    );
    println!("============================\n");
}

pub fn show_dashboard(user: &User) {
    let today_cals = crate::logic::get_calorie_sum(user, 0);
    let week_cals = crate::logic::get_calorie_sum(user, 6);
    let month_cals = crate::logic::get_calorie_sum(user, 29);
    let today_burned = crate::logic::get_activity_sum(user, 0);
    let net_today = today_cals as i32 - today_burned as i32;
    let remaining = user.dri as i32 - net_today;

    println!("\n=== Progress Report for {} ===", user.name.red());
    println!("Daily Target:  {} kcal", user.dri.to_string().red());
    println!("-------------------------------");
    println!(
        "Consumed Today:              {} kcal",
        today_cals.to_string().red()
    );
    println!(
        "Burnt Today From Activities: {} kcal",
        today_burned.to_string().red()
    );
    println!(
        "Net intake:                  {} kcal",
        net_today.to_string().red()
    );
    println!(
        "Remaining:                   {} kcal",
        remaining.to_string().red()
    );
    println!("-------------------------------");
    println!(
        "Last 7 Days:    {} kcal (avg: {}/day)",
        week_cals.to_string().red(),
        week_cals / 7
    );
    println!(
        "Last 30 Days:   {} kcal (avg: {}/day",
        month_cals.to_string().red(),
        month_cals / 30
    );
    println!("-------------------------------");
    println!("===============================\n");

    weight_chart(&user);
}

pub fn list_meals(user: &User) {
    println!("--- Your Meals ---");

    if user.meals.is_empty() {
        println!("{}", "No meals logged yet".red());
        return;
    }

    for (index, meal) in user.meals.iter().enumerate() {
        let date_str = meal.date.format("%b %d, %H:%M").to_string();

        println!(
            "[{}] {} - {} kcal (P:{}g, C:{}g, F:{}g",
            index.to_string().red(),
            date_str.red(),
            meal.kcal.to_string().red(),
            meal.protein.to_string().red(),
            meal.carbs.to_string().red(),
            meal.fat.to_string().red()
        );
    }
    println!("-------------------------------");
}

pub fn list_activities(user: &User) {
    if user.activities.is_empty() {
        println!("{}", "No activities logged yet".red());
        return;
    }

    for (index, act) in user.activities.iter().enumerate() {
        let date_str = act.date.format("%b %d, %H:%M").to_string();

        println!(
            "[{}] {} - {}: {} kcal burned",
            index.to_string().red(),
            date_str.to_string().red(),
            act.act_type.red(),
            act.kcal_burn.to_string().red()
        );
    }
    println!("-------------------------------");
}

fn weight_chart(user: &User) {
    println!("\n--- Weight Progress Chart ---");
    if user.weights.is_empty() {
        println!("No weight history recorded yet.");
        return;
    }
    let mut all_weights: Vec<f32> = user.weights.iter().map(|(_, w)| *w).collect();
    all_weights.push(user.goal_weight);

    let max_w = all_weights.iter().copied().fold(f32::MIN, f32::max);
    let min_w = all_weights.iter().copied().fold(f32::MAX, f32::min);
    let range = max_w - min_w;
    let chart_width = 40.0;

    for (date, weight) in &user.weights {
        let normalized_len = if range > 0.1 {
            ((weight - min_w) / range) * chart_width
        } else {
            chart_width / 2.0
        };

        let bar_len = (normalized_len as usize).max(1);
        let bar = "█".repeat(bar_len).red();

        println!(
            "{}: {:>5.1} kg {}",
            date.format("%b %d").to_string().red(),
            weight,
            bar
        );

        println!(
            "{}",
            "------------------------------------------".bright_black()
        );

        let current_weight = user.weight;
        let diff = current_weight - user.goal_weight;
        if diff > 0.0 {
            println!("{} kg to go!", format!("{:.1}", diff).red());
        } else {
            println!("{}", "Goal weight reached!".green().bold());
        }
    }
}
