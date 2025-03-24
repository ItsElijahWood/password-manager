use std::io;

use rusqlite::{Connection, Result};

pub fn login_print(user: &str) {
  let id = check_id(user).unwrap();
  let listener = io::stdin();

  let mut input = String::new();

  println!("
      Commands: 
        Create Password
        List Passwords
  ");

  println!("Input: ");

  listener.read_line(&mut input).expect("Error reading line in login_print.rs");

  let input = input.trim().to_lowercase();

  loop {
    match input.as_str() {
      "create password" => {
        
        println!("Password: ");

        let mut input = String::new();
        listener.read_line(&mut input).expect("Error reading line in login_print.rs");

        if let Err(e) = create_password(id.to_string(), input.to_string()) {
          eprintln!("Error creating password: {}", e);
        } else {
          println!("Created password successfully.");
        };
      }
      "list passwords" => {
        let passwords = list_password(id.to_string()).unwrap();

        for password in passwords {
          println!("{}", password);
        }
        break;
      }
      _ => println!("Command not found.")
    }
  }
}

fn check_id(user: &str) -> Result<usize> {
  let connection = Connection::open("passwords.db")?;

  let mut stmt = connection.prepare("SELECT id FROM users WHERE user = ?")?;

  let id: usize = stmt.query_row([user], |row| row.get::<_, usize>(0))?;

  Ok(id)
}

fn create_password(id: String, password: String) -> Result<()> {
  let connection = Connection::open("passwords.db")?;

  connection.execute("INSERT INTO passwords (id, password) VALUES (?, ?)", [id, password])?;

  Ok(())
}

fn list_password(id: String) -> Result<Vec<String>> {
  let connection = Connection::open("passwords.db")?;

  let mut stmt = connection.prepare("SELECT password FROM passwords WHERE id = ?")?;
  let mut rows = stmt.query([id])?;

  let mut passwords = Vec::new();

  while let Some(row) = rows.next()? {
    let password: String = row.get(0)?;

    passwords.push(password);
  }

  Ok(passwords)
}
