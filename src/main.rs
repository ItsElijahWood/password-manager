use std::io;

mod modules;

fn main() {
    if let Err(e) = modules::init_database::init_database() {
        eprintln!("Error creating database: {}", e);
    };
    
    let listener = io::stdin();

    println!("Commands:
        Create Account
        Login
        Delete Database
    ");

    println!("Input:");

    let mut input = String::new();

    listener.read_line(&mut input).expect("Error reading input.");

    if input.trim() == "create account" {
        
    } else if input.trim() == "login" {

    } else if input.trim() == "delete database" {
        if let Err(e) = modules::delete_database::delete_database() {
            eprintln!("Failed to delete database with error: {}", e);
        };
    }
}
