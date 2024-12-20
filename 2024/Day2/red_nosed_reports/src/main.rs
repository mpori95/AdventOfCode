mod handlers;

use std::{fs, io::{self, BufRead}};
use handlers::red_nosed_report_context::RedNosedReportContext;

fn main() {
    siphon_safe_reports();
}

fn siphon_safe_reports(){
    println!("Collecting safe reports...");

    // Open the unusual data file
    match fs::File::open("unusual_data.txt"){
        Ok(file) => {

            // Define buffer to start reading each line of the file
            let reader = io::BufReader::new(file);

            let mut safe_report_count = 0;

            // Loop through each report to analyze
            for line in reader.lines(){

                // Check if the report is safe
                match line {
                    Ok(report) => {

                        let levels: Vec<i32> = report
                            .split_whitespace()
                            .map(|s| s
                                .parse()
                                .unwrap())
                            .collect();

                        let increasing_levels = levels.windows(2).all(|level| 
                            level[0] < level[1] && (level[1] - level[0]) <= 3);

                        let decreasing_levels = levels.windows(2).all(|level|
                            level[0] > level[1] && (level[0] - level[1]) <= 3);

                        if increasing_levels || decreasing_levels{
                            safe_report_count += 1;
                            continue;
                        }

                        let report_length = levels.len();
                        let mut level_position = 0;

                        while level_position < report_length{

                            let mut unsafe_levels = levels.clone();
                            unsafe_levels.remove(level_position);

                            let is_increasing = unsafe_levels.windows(2).all(|level|
                                level[0] < level[1] && (level[1] - level[0]) <= 3);

                            
                            let is_decreasing = unsafe_levels.windows(2).all(|level|
                                level[0] > level[1] && (level[0] - level[1]) <= 3);

                            if is_increasing || is_decreasing {
                                safe_report_count += 1;
                                break;
                            }

                            level_position += 1;

                        }
                    }
                    Err(error) => {
                        println!("Error occurred collecting safe reports ({error})")
                    }
                }
            }

            println!("{}", safe_report_count);

        } Err(error) => {
            
        }
    }
}
