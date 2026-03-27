
#  Rust Fitness CLI (v1.0)

A high-performance, terminal-based fitness and calorie tracker built in Rust. This tool helps you calculate your BMR, set daily intake goals, and track your progress without the need for a bulky database.

##  Features

- **No Database Required**: All user data is stored in simple, portable `.json` files.
- **BMR & DRI Calculation**: Uses the Harris-Benedict formula to calculate your metabolic rate and daily calorie needs.
- **Multi-User Support**: Create and manage separate profiles for different users.
- **Privacy First**: Your data stays on your machine in human-readable format.
- **Dual System Support**: Works with both Metric (kg/cm) and Imperial (lb/in) units.
- **Smart Adaptive DRI**: Automatically adjusts your daily calorie target based on your current weight vs. your goal weight (sustainable deficit/surplus logic).
- **Interactive UI**: A centered, color-coded dashboard that updates in real-time.
- **Activity & Step Converter**: Log activities manually or convert walking steps directly into burned calories.
- **Progress Tracking**: View a visual ASCII chart of your weight journey over time.

##  Installation from source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ZanderwKuhne/rust-cli-fitness.git
   cd rust-fitness-cli
   cargo build --release

2. Run it:
   ```bash
   ./target/release/rust_fitness_cli


##  Docker Support

Run the application in a containerized environment to keep your system clean.

### **1. Run the following:**
   ```bash
   docker run -it -v $(pwd):/app/data zanderkuhne/rust-cli-fitness:latest
```


### **Managing Data**
User profiles are saved as `{name}.json` in the project root. You can manually backup these files or delete them to reset a profile.

