use std::{fs, io::{self, BufRead}};

fn main() {
    sort_locations();
}

fn sort_locations(){
    println!("Sorting locations...");

    // Open the location id file
    match fs::File::open("location_ids.txt"){
        Ok(f) => {
            
            // Define buffer to start reading each line of the file
            let reader = io::BufReader::new(f);

            // Create vectors to hold data from the file
            let mut first_column: Vec<i32> = Vec::new();
            let mut second_column: Vec<i32> = Vec::new();

            // Loop through each line using the buffer
            for line in reader.lines(){

                // Collect the two ids from each line from the file
                match line {
                    Ok(content) => {
                        let test: Vec<&str> = content.split("   ").collect();
                        let first_location_id = test[0].parse().unwrap();
                        let second_location_id = test[1].parse().unwrap();

                        // Add values to vectors
                        first_column.push(first_location_id);
                        second_column.push(second_location_id);
                        
                    }
                    Err(error) => {
                        println!("Error collecting location Ids from each line. Error: {error}")
                    }
                }
            }
            
            first_column.sort();
            second_column.sort();

            let mut i = 0;
            let mut running_total = 0;
            let list_length = first_column.len();

            while i < list_length{
                
                let left_value = first_column[i];
                let right_value = second_column[i];

                let difference = if left_value > right_value 
                {left_value - right_value} else 
                {right_value - left_value};

                running_total += difference;

                i += 1;
            }

            let mut similarity_score = 0;


            for left_value in first_column{

                let mut occurrences = 0;

                for right_value in &second_column {

                    if left_value == *right_value{
                        occurrences += 1;
                    }

                }

                similarity_score += left_value * occurrences;

            }

            println!("{similarity_score}");

            println!("{running_total}");

        }
        Err(error) => {

        }
    };
}

// fn add_locations(line: Result<String, std::io::Error>){


// }

