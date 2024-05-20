use rusqlite::{Connection, Result};
use std::io::stdin;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    master_password: String,
}

fn display_options_list() {
    const SCREEN_TEXT: [&str; 4] = [
        "PASSWORD MANAGER :",
        "1. Create Account",
        "2. Login",
        "3. Add Password",
    ];
    for text in SCREEN_TEXT {
        println!("{text}");
    }
}

fn create_account() {}

fn login() {}

fn add_password() {}

fn password_manager_operations(selected_operation: u8, logged: bool) {}

fn main() -> Result<()> {
    let path = "./my_db.db3";
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users ( id INTEGER PRIMARY KEY, user_name TEXT NOT NULL UNIQUE, master_password TEXT NOT NULL)",
        (),
    )?;

    let user1 = User {
        id: 0,
        name: "user1".to_string(),
        master_password: "password".to_string(),
    };

    conn.execute(
        "INSERT INTO users (user_name, master_password) VALUES (?1, ?2)",
        (&user1.name, &user1.master_password),
    )?;

    let mut stmt = conn.prepare("SELECT id, user_name, master_password FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            master_password: row.get(2)?,
        })
    })?;

    for user in user_iter {
        println!("Found users {:?}", user.unwrap());
    }

    let mut user_input = String::new();
    loop {
        display_options_list();
        stdin()
            .read_line(&mut user_input)
            .expect("Error while reading line");
        println!("{}", user_input.trim());
        user_input.clear();
    }

    Ok(())
}
