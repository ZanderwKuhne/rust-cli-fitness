use std::usize;

// Module handling display logic
use crate::users::User;
use colored::*;
use crossterm::terminal::size;

pub fn show_user(user: &User) {
    let header_plain = format!("=== Profile for {} ===", user.name);
    let header_colored = format!("=== Profile for {} ===", user.name.red()).normal();
    print_centered_colored(&header_plain, header_colored.into());

    let age_gen_plain = format!("Age: {} | Gender: {}", user.age, user.gender);
    let age_gen_colored = format!(
        "Age: {} | Gender: {}",
        user.age.to_string().red(),
        user.gender.red()
    )
    .normal();
    print_centered_colored(&age_gen_plain, age_gen_colored.into());

    let w_h_plain = format!(
        "Weight: {:.1} kg | Height: {:.1} cm",
        user.weight, user.height
    );
    let w_h_colored = format!(
        "Weight: {} kg | Height: {} cm",
        format!("{:.1}", user.weight).red(),
        format!("{:.1}", user.height).red()
    )
    .normal();
    print_centered_colored(&w_h_plain, w_h_colored.into());

    print_centered("----------------------------");

    let bmr_plain = format!("BMR: {:.2} kcal", user.bmr);
    let bmr_colored = format!("BMR: {} kcal", format!("{:.2}", user.bmr).red()).normal();
    print_centered_colored(&bmr_plain, bmr_colored.into());

    let dri_plain = format!("Daily Target (DRI): {:.2} kcal", user.dri);
    let dri_colored = format!(
        "Daily Target (DRI): {} kcal",
        format!("{:.2}", user.dri).red()
    )
    .normal();
    print_centered_colored(&dri_plain, dri_colored.into());

    print_centered("============================");
}

pub fn show_dashboard(user: &User) {
    let today_cals = crate::logic::get_calorie_sum(user, 0);
    let today_burned = crate::logic::get_activity_sum(user, 0);
    let net_today = today_cals as i32 - today_burned as i32;
    let remaining = user.dri as i32 - net_today;

    center_vertically(18);
    print_centered(&format!("=== Progress Report for {} ===", user.name.trim()));
    print_centered("-------------------------------");

    center_row("Daily Target:  ", &format!("{} kcal", user.dri));
    center_row("Consumed Today: ", &format!("{} kcal", today_cals));
    center_row("Burnt Today:    ", &format!("{} kcal", today_burned));

    let rem_color = if remaining >= 0 {
        remaining.to_string().green()
    } else {
        remaining.to_string().red()
    };
    let rem_plain = format!("Remaining:      {} kcal", remaining);
    let rem_colored = format!("Remaining:      {} kcal", rem_color).normal();
    print_centered_colored(&rem_plain, rem_colored.into());

    print_centered("-------------------------------");

    weight_chart(user);
}

fn center_row(label: &str, value: &str) {
    let plain = format!("{}{}", label, value);
    let colored = format!("{}{}", label, value.red()).normal();
    print_centered_colored(&plain, colored.into());
}

pub fn list_meals(user: &User) {
    center_vertically(12);
    print_centered(
        &format!("--- {}'s Meal Log ---", user.name.trim())
            .cyan()
            .bold()
            .to_string(),
    );

    if user.meals.is_empty() {
        print_centered("No meals logged yet.");
        return;
    }

    // Show only the last 10 meals to keep the UI clean
    let start_idx = if user.meals.len() > 10 {
        user.meals.len() - 10
    } else {
        0
    };

    for (index, meal) in user.meals.iter().enumerate().skip(start_idx) {
        let date_str = meal.date.format("%b %d %H:%M").to_string();

        let plain = format!(
            "[{}] {} - {} kcal (P:{}g, C:{}g, F:{}g)",
            index, date_str, meal.kcal, meal.protein, meal.carbs, meal.fat
        );

        let colored = format!(
            "[{}] {} - {} kcal (P:{}g, C:{}g, F:{}g)",
            index.to_string().yellow(),
            date_str.bright_black(),
            meal.kcal.to_string().red(),
            meal.protein.to_string().green(),
            meal.carbs.to_string().blue(),
            meal.fat.to_string().magenta()
        )
        .normal();

        print_centered_colored(&plain, colored.into());
    }
}

pub fn list_activities(user: &User) {
    center_vertically(12);
    print_centered(
        &format!("--- {}'s Activity Log ---", user.name.trim())
            .magenta()
            .bold()
            .to_string(),
    );

    if user.activities.is_empty() {
        print_centered("No activities logged yet.");
        return;
    }

    let start_idx = if user.activities.len() > 10 {
        user.activities.len() - 10
    } else {
        0
    };

    for (index, act) in user.activities.iter().enumerate().skip(start_idx) {
        let date_str = act.date.format("%b %d %H:%M").to_string();

        let plain = format!(
            "[{}] {} - {}: {} kcal",
            index, date_str, act.act_type, act.kcal_burn
        );

        let colored = format!(
            "[{}] {} - {}: {} kcal",
            index.to_string().yellow(),
            date_str.bright_black(),
            act.act_type.cyan(),
            act.kcal_burn.to_string().magenta()
        )
        .normal();

        print_centered_colored(&plain, colored.into());
    }
}

pub fn list_weights(user: &User) {
    center_vertically(8);
    print_centered(
        &format!("--- {}'s Weight History ---", user.name)
            .cyan()
            .bold()
            .to_string(),
    );

    if user.weights.is_empty() {
        print_centered("No weight history recorded yet.");
        return;
    }

    for (index, (date, weight)) in user.weights.iter().enumerate() {
        let date_str = date.format("%b %d").to_string();
        let plain = format!("[{}] {} - {:.1} kg", index, date_str, weight);
        let colored = format!(
            "[{}] {} - {} kg",
            index.to_string().yellow(),
            date_str.bright_black(),
            format!("{:.1}", weight).red()
        )
        .normal();

        print_centered_colored(&plain, colored.into());
    }
}

fn weight_chart(user: &User) {
    print_centered("--- Weight Progress Chart ---");

    if user.weights.is_empty() {
        print_centered("No weight history recorded yet.");
        return;
    }

    let mut all_weights: Vec<f32> = user.weights.iter().map(|(_, w)| *w).collect();
    all_weights.push(user.goal_weight);
    let max_w = all_weights.iter().copied().fold(f32::MIN, f32::max);
    let min_w = all_weights.iter().copied().fold(f32::MAX, f32::min);
    let range = max_w - min_w;
    let chart_width = 30.0;
    for (date, weight) in &user.weights {
        let normalized_len = if range > 0.1 {
            ((weight - min_w) / range) * chart_width
        } else {
            chart_width / 2.0
        };
        let bar_len = (normalized_len as usize).max(1);
        let bar = "█".repeat(bar_len);
        let date_str = date.format("%b %d").to_string();

        let plain = format!("{}: {:>5.1} kg {}", date_str, weight, bar);
        let colored = format!(
            "{}: {:>5.1} kg {}",
            date_str.bright_black(),
            weight,
            bar.red()
        )
        .normal();

        print_centered_colored(&plain, colored.into());
    }

    print_centered("------------------------------------------");

    let current_weight = user.weight;
    let diff = current_weight - user.goal_weight;

    if diff > 0.0 {
        let diff_str = format!("{:.1} kg to go!", diff);
        print_centered_colored(&diff_str, diff_str.cyan().into());
    } else {
        let win_msg = "Goal weight reached!";
        print_centered_colored(win_msg, win_msg.green().bold().into());
    }
}

pub fn print_centered(text: &str) {
    let clean_text = text.trim_matches(|c| c == '\n' || c == '\r');
    let (width, _) = size().unwrap_or((80, 24));
    let text_len = clean_text.chars().count();

    let padding = if (width as usize) > text_len {
        (width as usize - text_len) / 2
    } else {
        0
    };
    println!("{}{}", " ".repeat(padding), clean_text);
}

pub fn print_centered_colored(text: &str, colored_text: colored::ColoredString) {
    println!("{}{}", get_padding(text.len()), colored_text);
}

pub fn center_vertically(content_height: u16) {
    let (_, height) = size().unwrap_or((80, 24));
    if height > content_height {
        let top_padding = (height - content_height) / 2;
        for _ in 0..top_padding {
            println!();
        }
    }
}

fn get_padding(text_len: usize) -> String {
    let (width, _) = size().unwrap_or((80, 24));
    let padding_size = if text_len < width as usize {
        (width as usize - text_len) / 2
    } else {
        0
    };
    " ".repeat(padding_size)
}
