use std::{fs, path::Path};

pub fn delete_database() -> std::io::Result<()> {
  let db_file = "passwords.db";

  if Path::new(db_file).exists() {
    fs::remove_file(db_file)?;
  };

  println!("Successfully deleted the database.");

  Ok(())
}
