use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;
use dirs::home_dir;

struct Task {
    name: String,
    description : String,
    finished: bool
}

fn main() {
    // Initalize application:
    create_application_folder();

    run_application();
}

fn run_application() {
    println!("To-Do-List-Application\n");
    println!("Choose what u want to do:");
    println!("1 --> Create a Todo-List");
    println!("2 --> Show already existing Todo-Lists");
    println!("3 --> Open a Todo-List");
    println!("4 --> Delete a Todo-List");
    println!("Wrong input will result in exiting the application!\n");

    // Get user_input:
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line!");

    // Format user_input:
    let user_input : i32 = user_input.trim().parse().expect("Not a valid number!");

    match user_input {
        1 => {create_to_do_list();}
        2 => {show_to_do_lists();}
        3 => {open_to_do_list();}
        4 => {delete_to_do_list();}
        5 => {add_to_do(String::from("MeinTest"));}
        _ => {exit(0);}
    }
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
    println!("\nEnter the name of the to-do list:");
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
    println!();
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
    println!(); 
    let dir = get_application_folder();

    // Get user_input:
    println!("\nEnter the name of the to-do list to open:");
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

fn delete_to_do_list() {
    println!(); 
    let dir = get_application_folder();

    // Get user_input:
    println!("\nEnter the name of the to-do list to delete:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Could not read line!");

    // Format user_input:
    let name = name.trim();
    let mut file_path = dir.join(name);
    file_path.set_extension("txt");

    if file_path.exists() {
        _ = fs::remove_file(file_path);
        println!("Sucessfully removed file from directory!");
    } else {
        println!("To-do list not found: {}", name);
    }
}

fn add_to_do(filename: String) {
    let dir = get_application_folder();

    // Build file_path:
    let filename = filename.trim();
    let mut file_path = dir.join(filename);
    file_path.set_extension("txt");

    // Open file with append option:
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Could not open file!");

    // Get user_input:
    println!("Enter the name of the task:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Could not read_line!");

    println!("Enter the descripton of the task:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Could not read_line!");

    // Create task:
    let task = Task {
        name: name,
        description: description,
        finished: false
    };

    file.write(task.name.as_bytes()).expect("Write failed!");
    file.write(task.description.as_bytes()).expect("Write failed!");
    file.write(task.finished.to_string().as_bytes()).expect("Write failed!");

    println!("Appended task to file!");


}

fn update_to_do() {

}

fn delete_to_do() {

}

fn get_application_folder() -> PathBuf{
    // Get home_dir:
    let mut dir = home_dir().expect("Could not find home directory!");

    // Append path to application directory:
    dir.push(".to-do-lists");

    return dir;
}