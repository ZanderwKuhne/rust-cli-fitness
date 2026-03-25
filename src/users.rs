// User and user related structs
// Serde to store and retrieve struct data in
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub system: bool,
    pub age: u8,
    pub act_level: u8,
    pub weight: f32,
    pub height: f32,
    pub goal_weight: f32,
    pub bmr: f32,
    pub dri: f32,
    pub gender: String,
    pub name: String,
    pub date: DateTime<Local>,
    pub meals: Vec<LogMeal>,
    pub activities: Vec<LogActivity>,
}

#[derive(Serialize, Deserialize)]
pub struct LogMeal {
    pub kcal: u32,
    pub protein: u32,
    pub fat: u32,
    pub carbs: u32,
    pub date: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct LogActivity {
    pub act_type: String,
    pub kcal_burn: u32,
    pub date: DateTime<Local>,
}
