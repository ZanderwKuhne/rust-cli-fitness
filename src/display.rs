use crate::users::User;

pub fn show_user(user: &User) {
    println!("\n=== Profile for {} ===", user.name);
    println!("Age:    {} | Gender: {}", user.age, user.gender);
    println!(
        "Weight: {:.1} kg | Height: {:.1} cm",
        user.weight, user.height
    );
    println!("----------------------------");
    println!("BMR:    {:.2} kcal", user.bmr);
    println!("Daily Target (DRI): {:.2} kcal", user.dri);
    println!("============================\n");
}

pub fn show_dashboard(user: &User) {
    let today_cals = crate::logic::get_calorie_sum(user, 0);
    let week_cals = crate::logic::get_calorie_sum(user, 6);
    let month_cals = crate::logic::get_calorie_sum(user, 29);
    let today_burned = crate::logic::get_activity_sum(user, 0);
    let net_today = today_cals as i32 - today_burned as i32;

    println!("\n=== Progress Report for {} ===", user.name);
    println!("Daily Target:  {} kcal", user.dri);
    println!("-------------------------------");
    println!("Consumed Today:              {} kcal", today_cals);
    println!("Burnt Today From Activities: {} kcal", today_burned);
    println!("Net intake:                  {} kcal", net_today);
    println!(
        "Remaining:                   {} kcal",
        user.dri as i32 - net_today
    );
    println!("-------------------------------");
    println!(
        "Last 7 Days:    {} kcal (avg: {}/day)",
        week_cals,
        week_cals / 7
    );
    println!("Last 30 Days:   {} kcal", month_cals);
    println!("-------------------------------");
    println!("===============================\n");
}
