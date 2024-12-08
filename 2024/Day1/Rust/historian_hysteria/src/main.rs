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
            let first_column_size = first_column.len();
            let second_column_size = second_column.len();

            println!("First Column: {first_column_size}");
            println!("Second Column: {second_column_size}");

        }
        Err(error) => {

        }
    };
}

// fn add_locations(line: Result<String, std::io::Error>){


// }

