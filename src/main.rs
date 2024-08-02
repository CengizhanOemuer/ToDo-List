use std::fs::{self, File};
use std::io::{self, Write, BufRead, BufReader};
use std::path::{Path, PathBuf};
use dirs::home_dir;

fn main() {
    // Initalize application:
    create_application_folder();

    // Test create-to-do-list:
    create_to_do_list();

    // Test show-to-do-lists:
    show_to_do_lists();

    // Test open-to-do-list:
    open_to_do_list();
}

fn create_application_folder() {
    let dir = get_application_folder();

    // Check wheter directory does not exists:
    if !dir.exists() {
        // Create directory:
        fs::create_dir(&dir).expect("Could not create the application directory!");
        println!("Successfully created application directory!")
    }
}

fn create_to_do_list() {
    let dir = get_application_folder();
    
    // Get user_input:
    println!("Enter the name of the to-do list:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line!");
    let name = name.trim();

    // Create file_path:
    let mut file_path = dir.join(name);
    file_path.set_extension("txt");

    // Create file:
    let _file = File::create(&file_path).expect("Could not create file!");
    
    println!("Created to-do list: {}", name);
}

fn show_to_do_lists() {
    let dir = get_application_folder();

    // Read files from directory:
    if let Ok(files) = fs::read_dir(dir) {
        // Iterate through files in directory:
        for file in files {
            let file = file.expect("Could not read file from directory!");

            // Get file_name from file:
            let path = file.path();
            if let Some(file_name) = path.file_name() {
                if let Some(file_str) = file_name.to_str() {
                    println!("{}", file_str);
                }
            }
        }
    } else {
        println!("No to-do lists found!");
    }
}

fn open_to_do_list() {
    let dir = get_application_folder();

    // Get user_input:
    println!("Enter the name of the to-do list to open:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line!");

    // Format user_input:
    let name = name.trim();
    let mut file_path = dir.join(name);
    file_path.set_extension("txt");

    if file_path.exists() {
        let file = File::open(&file_path).expect("Could not open file!");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line.expect("Could not read line!"));
        }
    } else {
        println!("To-do list not found: {}", name);
    }
}

fn get_application_folder() -> PathBuf{
    // Get home_dir:
    let mut dir = home_dir().expect("Could not find home directory!");

    // Append path to application directory:
    dir.push(".to-do-lists");

    return dir;
}