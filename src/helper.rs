// Added some helper functions to clean up main
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    io,
    io::{Write, stdout},
};

pub fn get_input(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<u32>().unwrap_or(0)
}

pub fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}

pub fn pause() {
    println!("\nPress Enter to return...");
    let mut _unused = String::new();
    io::stdin().read_line(&mut _unused).unwrap();
}
