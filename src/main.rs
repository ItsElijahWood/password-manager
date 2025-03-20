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
        List Users
    ");

    println!("Input:");

    let mut input = String::new();

    listener.read_line(&mut input).expect("Error reading input.");

    if input.trim() == "create account" {
        let mut user = String::new();
        let mut password = String::new();

        println!("User: ");

        listener.read_line(&mut user).expect("Error reading line on user input.");

        println!("Password: ");

        listener.read_line(&mut password).expect("Error reading line on user input.");

        if let Err(e) = modules::create_account::create_accout(&user, &password) {
            eprintln!("Failed to insert values into database when signing up: {}", e);
        } else {
            println!("Successfully inserted user into the database.");
        }
    } else if input.trim() == "login" {

    } else if input.trim() == "delete database" {
        if let Err(e) = modules::delete_database::delete_database() {
            eprintln!("Failed to delete database with error: {}", e);
        };
    } else if input.trim() == "list users" {
        if let Err(e) = modules::list_users::list_users() {
            eprintln!("Error listing users: {}", e);
        }
    }
}
