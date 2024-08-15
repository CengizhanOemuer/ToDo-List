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
    loop {
        println!("To-Do-List-Application\n");
        println!("Choose what u want to do:");
        println!("1 --> Create a Todo-List");
        println!("2 --> Open a Todo-List");
        println!("3 --> Delete a Todo-List");
        println!("4 --> Show already existing Todo-Lists");
        println!("Wrong input will result in exiting the application!\n");

        // Get user_input:
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line!");

        // Format user_input:
        let user_input : i32 = user_input.trim().parse().expect("Not a valid number!");

        match user_input {
            1 => {create_to_do_list();}
            2 => {
                let file = open_to_do_list().unwrap();
                loop {
                    println!("\n Choose between:\n1 --> Add a task\n2 --> Update a task\n3 --> Close Todo-List");

                    // Get user_input:
                    let mut user_input = String::new();
                    io::stdin().read_line(&mut user_input).expect("Failed to read line!");

                    // Format user_input:
                    let user_input : i32 = user_input.trim().parse().expect("Not a valid number!");

                    match user_input {
                        1 => {add_to_do(&file);}
                        2 => {update_to_do(&file);}
                        3 => {
                            println!("Closed Todo-List!");
                            break;
                        }
                        _ => {}
                    }
                }
            }
            3 => {delete_to_do_list();}
            4 => {show_to_do_lists();}
            _ => {exit(0);}
        }
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

fn open_to_do_list() -> Option<PathBuf> {
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
        Some(file_path)
    } else {
        println!("To-do list not found: {}", name);
        None
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

fn add_to_do(file_path: &PathBuf) {
    let dir = get_application_folder();

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
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        finished: false
    };

     // Format task as XML:
     let xml_data = format!(
        "<task>\n   <name>{}</name>\n   <description>{}</description>\n   <finished>{}</finished>\n</task>\n",
        task.name,
        task.description,
        task.finished
    );

    file.write_all(xml_data.as_bytes()).expect("Could not write in file!");

    println!("Appended task to file!");
}

fn update_to_do(file_path: &PathBuf) {
    // Open file:
    let file = File::open(file_path).expect("Could not open file!");
    let reader = BufReader::new(file);

    // Read lines into a vector:
    let mut lines: Vec<String> = reader.lines().map(|l| l.expect("Could not read line")).collect();

    // Get the name of the task to update:
    println!("Enter the name of the task to mark it as finished:");
    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name).expect("Failed to read line!");
    let task_name = task_name.trim();

    // Flag to indicate if the task was found and updated:
    let mut task_found = false;

    // Iterate through the lines and update the task's finished status:
    for i in 0..lines.len() {
        if lines[i].trim() == format!("<name>{}</name>", task_name) {
            // Find the next line with <finished> and set it to true:
            for j in i + 1..lines.len() {
                if lines[j].trim().starts_with("<finished>") {
                    lines[j] = "   <finished>true</finished>".to_string();
                    task_found = true;
                    break;
                }
            }
        }
        if task_found {
            break;
        }
    }

    // If the task was found and updated, write the lines back to the file
    if task_found {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .expect("Could not open file for writing!");

        for line in lines {
            file.write_all(line.as_bytes()).expect("Could not write to file");
            file.write_all(b"\n").expect("Could not write newline");
        }

        println!("Task '{}' marked as finished!", task_name);
    } else {
        println!("Task '{}' not found!", task_name);
    }
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