// DONE Run the app
// DONE Create a test csv file
// DONE Read a csv - hardcode file
// Accept path via cli
// update IDE to cargo fmt automatically
use std::{
    fs::File,
    io::{Read, Result},
};

fn read_file(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    // check if arg is path then use to get csv content
    let csv_content = match read_file("./test.csv") {
        Ok(contents) => contents,
        Err(error) => panic! {"fail {}", error},
    };
    println!("{csv_content}\n\n{args:#?}");
}
