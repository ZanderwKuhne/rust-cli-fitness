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
