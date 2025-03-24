use rusqlite::{Connection, Result};

pub fn login(user: &str, password: &str) -> Result<bool> {
  check_login_details(user, password) 
}

fn check_login_details(user: &str, password: &str) -> Result<bool> {
  let connection = Connection::open("passwords.db")?;

  let mut stmt = connection.prepare("SELECT password FROM users WHERE user = ?")?;

  let db_password: String = match stmt.query_row([user], |row| row.get::<_, String>(0)) {
    Ok(pass) => pass,
    Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(false),
    Err(e) => return Err(e)
  };

  Ok(db_password == password)
}
