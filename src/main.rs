use std::fs::{self, File};
use std::io::{self, Write, BufRead, BufReader};
use std::path::{Path, PathBuf};
use dirs::home_dir;

fn main() {
    // Initalize application:
    create_application_folder();

}

fn create_application_folder() {
    // Get home_dir:
    let mut dir = home_dir().expect("Could not find home directory!");

    // Append path to application dir:
    dir.push(".to-do-lists");

    // Check wheter directory does not exists:
    if !dir.exists() {
        // Create directory:
        fs::create_dir(&dir).expect("Could not create the application directory!");
        println!("Successfully created application directory!")
    }
}