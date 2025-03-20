use rusqlite::{Connection, Result};

pub fn list_users() -> Result<()> {
  let connection = Connection::open("passwords.db")?;

  let mut stmt = connection.prepare(
    "SELECT COUNT(*) FROM users",
  )?;

  let get_stmt = stmt.query_map([], |row| row.get::<_, usize>(0))?;

  for user in get_stmt {
    println!("{:#?}", user);
  }

  Ok(())
}