use std::env;

fn main() {
    println!("Hello, world!");
    
    let args: Vec<String> = env::args().collect();

    dbg!(args);
}

fn parse_arguments(args: Vec<&str>) {
    let command = args[1];

    match command {
        "add" => {
            
        }, 
        "update" => {

        },
        "delete" => {

        },
        "show" => {

        },
        "help" => {

        },
        &_ => {

        }
    }
}
