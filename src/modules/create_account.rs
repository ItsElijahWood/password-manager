use rusqlite::{Connection, Result};

pub fn create_accout(user: &str, password: &str) -> Result<()> {
  let connection = Connection::open("passwords.db")?;

  let mut stmt = connection.prepare(
    "INSERT INTO users (user, password) VALUES (?, ?)",
  )?;

  stmt.execute([user, password])?;

  Ok(())
}
