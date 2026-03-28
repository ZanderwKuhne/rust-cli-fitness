mod calc;
mod display;
mod helper;
mod logic;
mod users;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

fn main() {
    execute!(stdout(), EnterAlternateScreen).expect("Failed to enter alternate screen");
    logic::landing_page();
    execute!(stdout(), LeaveAlternateScreen).expect("Failed to leave alternate screen");
}
