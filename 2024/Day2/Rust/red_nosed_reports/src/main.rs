use std::{fs, io::{self, BufRead}};

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

                        // Collect each report as a vector of integers
                        let levels: Vec<i32> = report
                            .split_whitespace()
                            .map(|s| s
                                .parse()
                                .unwrap())
                            .collect();

                        // Check if the report is safe and continue loop if so
                        if is_report_safe(&levels){
                            safe_report_count += 1;
                            continue;
                        }

                        // If report was unsafe, see if it is safe with a level removed
                        let report_length = levels.len();

                        let mut level_position = 0;

                        while level_position < report_length{

                            let mut unsafe_levels = levels.clone();
                            unsafe_levels.remove(level_position);

                            if is_report_safe(&unsafe_levels){
                                safe_report_count += 1;
                                break;
                            }

                            level_position += 1;

                        }
                    }
                    Err(error) => {
                        println!("Error occurred collecting safe reports ({error})");
                    }
                }
            }

            println!("{}", safe_report_count);

        } Err(error) => {
            println!("Error opening report ({error})");
        }
    }
}

// Returns a bool that determines if the report is safe or not.
fn is_report_safe(report: &Vec<i32>) -> bool {

    let is_increasing = report.windows(2).all(|level|
        level[0] < level[1] && (level[1] - level[0]) <= 3);

    let is_decreasing = report.windows(2).all(|level|
        level[0] > level[1] && (level[0] - level[1]) <= 3);

    return is_increasing || is_decreasing

}
