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

                        let levels: Vec<i32> = report
                            .split_whitespace()
                            .map(|s| s
                                .parse()
                                .unwrap())
                            .collect();

                            let mut levels_count = 0;

                        while (levels_count + 1 <= levels.len()){

                            if levels[levels_count] == levels[levels_count + 1] {
                                return;
                            }
                            else if levels[levels_count] > levels[levels_count + 1] {

                            }
                        }


                    }
                    Err(error) => {

                    }
                }
            }

        } Err(error) => {
            
        }
    }
}
