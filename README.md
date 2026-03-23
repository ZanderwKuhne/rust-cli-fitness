
#  Rust Fitness CLI

A high-performance, terminal-based fitness and calorie tracker built in Rust. This tool helps you calculate your BMR, set daily intake goals, and track your progress without the need for a bulky database.

##  Features

- **No Database Required**: All user data is stored in simple, portable `.json` files.
- **BMR & DRI Calculation**: Uses the Harris-Benedict formula to calculate your metabolic rate and daily calorie needs.
- **Multi-User Support**: Create and manage separate profiles for different users.
- **Privacy First**: Your data stays on your machine in human-readable format.
- **Dual System Support**: Works with both Metric (kg/cm) and Imperial (lb/in) units.

##  Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com
   cd rust-fitness-cli

##  Docker Support

Run the application in a containerized environment to keep your system clean.

### **1. Build the image:**
```bash
docker build -t fitness-cli .
