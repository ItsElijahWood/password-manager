use rusqlite::{Connection, Result};

pub fn init_database() -> Result<()> {
  let connection = Connection::open("passwords.db")?;

  connection.execute(
    "create table if not exists passwords (
      id integer not null primary key,
      password text not null
    )", 
    (),
  )?;

   connection.execute("
    create table if not exists users (
      id integer primary key,
      user text not null,
      password text not null
    );
  ", (),
  )?;

  Ok(())
}