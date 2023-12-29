use core::num;
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

fn main() {

    let calibration_file = "./src/calibration.txt";

    let mut calibration_value: u32 = 0;

    match read_lines(calibration_file) {
        Ok(lines) => {

            let spelled_out_numbers: Vec<(&str, char)> = vec![("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')];

            for line in lines {

                let characters: Vec<char> = line.chars().collect();

                let mut first_value = None;
                let mut last_value = None;

                let mut found_number_words: Vec<(char, usize)> = Vec::new();

                for (number, char_value) in &spelled_out_numbers {

                    match line.find(number) {
                        Some(index) => {
                            found_number_words.push((*char_value, index));

                            let mut new_index = index;

                            while let Some(pos) = line[new_index..].find(number) {
                                let absolute_position = new_index + pos;

                                found_number_words.push((*char_value, absolute_position));

                                new_index = absolute_position + number.len();
                            }
                        }
                        None => {

                        }
                    }
                }

                found_number_words.sort_by_key(|a| a.1);
                    
                    let first = found_number_words.first();
                    let last = found_number_words.last();

                    match first {
                        Some((number_char, mut index)) => {
                            first_value = Some(number_char);

                            while index > 0 {
                                index -= 1;

                                if characters[index].is_numeric(){
                                    first_value = Some(&characters[index])
                                }
                            }
                        }
                        None => {
                            for c in &characters {
                                if c.is_numeric(){

                                    if first_value.is_none() {
                                        first_value = Some(c);
                                    }
                                }
                            }
                        }
                    }

                    match last {
                        Some((number_char, mut index)) => {
                            last_value = Some(number_char);

                            while index < line.len() {

                                if characters[index].is_numeric() {
                                    last_value = Some(&characters[index]);
                                }

                                index += 1;
                            }
                        }
                        None => {
                            for c in &characters {
                                if c.is_numeric(){
                                    last_value = Some(c);
                                }
                            }
                        }
                    }

                let combined_number = match (first_value, last_value) {
                    (Some(first), Some(last)) => format!("{}{}", first, last),
                    (Some(first), None) => format!("{}", first), // Only first found
                    (None, Some(last)) => format!("{}", last),   // Only last found
                    (None, None) => String::from("No numbers found"), // Neither found
                };

                println!("{}", combined_number);

                match combined_number.parse::<u32>() {
                    Ok(number) => {

                        calibration_value += number;
                        // println!("{}", calibration_value);
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