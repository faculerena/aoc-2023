#!/bin/bash

# Create the solutions module
mkdir src/solutions
touch src/solutions/mod.rs

# Create the utils module
mkdir src/utils
touch src/utils/mod.rs

# Generate 25 solution files and input.txt for each day
for day in {1..25}; do
  day_folder="day${day}"
  mkdir "src/solutions/${day_folder}"
  touch "src/solutions/${day_folder}/mod.rs"
  touch "src/solutions/${day_folder}/input.txt"

  echo "pub mod day${day};" >> src/solutions/mod.rs

  # Create the run function in each day's file
  echo "use std::fs;" >> "src/solutions/${day_folder}/mod.rs"
  echo "use std::path::Path;" >> "src/solutions/${day_folder}/mod.rs"
  echo "" >> "src/solutions/${day_folder}/mod.rs"
  echo "pub fn run() {" >> "src/solutions/${day_folder}/mod.rs"
  echo "    let input = fs::read_to_string(Path::new(\"input.txt\")).expect(\"Failed to read input file\");" >> "src/solutions/${day_folder}/mod.rs"
  echo "    // Your solution code here" >> "src/solutions/${day_folder}/mod.rs"
  echo "}" >> "src/solutions/${day_folder}/mod.rs"
done

# Create the main file
touch src/main.rs
echo "mod solutions;" >> src/main.rs
echo "mod utils;" >> src/main.rs
echo "" >> src/main.rs
echo "fn main() {" >> src/main.rs
echo "    // Choose which day to run" >> src/main.rs
echo "    solutions::day1::run();" >> src/main.rs
echo "    solutions::day2::run();" >> src/main.rs
# Repeat the line above for each day
echo "}" >> src/main.rs

echo "Project scaffolding generated successfully!"
