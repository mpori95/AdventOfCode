use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

fn main() {

    let calibration_file = "./src/calibration.txt";

    let mut calibration_value: u32 = 0;

    match read_lines(calibration_file) {
        Ok(lines) => {

            for line in lines {

                let line_characters = line.chars();

                let mut first_value = None;
                let mut last_value = None;

                for c in line_characters {
                    if c.is_numeric(){

                        if first_value.is_none(){
                            first_value = Some(c);
                        }

                        last_value = Some(c);

                    }
                }

                let combined_number = match (first_value, last_value) {
                    (Some(first), Some(last)) => format!("{}{}", first, last),
                    (Some(first), None) => format!("{}", first), // Only first found
                    (None, Some(last)) => format!("{}", last),   // Only last found
                    (None, None) => String::from("No numbers found"), // Neither found
                };

                match combined_number.parse::<u32>() {
                    Ok(number) => {

                        calibration_value += number;
                        println!("{}", calibration_value);
                    }
                    Err(er) => {
                        println!("Failed to parse '{}' as a number: {}", combined_number, er);
                    }
                };
            }
        }
        Err(e) => {
            println!("Error reading from file: {}", e);
        }
    };

    println!("{}", calibration_value);

}

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    reader.lines().collect()
}
