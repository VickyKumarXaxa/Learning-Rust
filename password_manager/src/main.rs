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

fn create_account(conn: &Connection) -> Result<()> {
    println!("Creating Account...");

    let mut name = String::new();
    let mut master_password = String::new();

    println!("Enter user name:");
    stdin()
        .read_line(&mut name)
        .expect("Failed to read username");
    name = name.trim().to_string();

    println!("Enter master password:");
    stdin()
        .read_line(&mut master_password)
        .expect("Failed to read master password");
    master_password = master_password.trim().to_string();

    conn.execute(
        "INSERT INTO users (user_name, master_password) VALUES (?1, ?2)",
        (&name, &master_password),
    )?;

    Ok(())
}

fn login() {}

fn add_password() {}

fn password_manager_operations(selected_operation: u8, conn: &Connection) {
    if selected_operation == 1 {
        // Create Account
        create_account(&conn);
    } else if selected_operation == 2 {
        // Login
        login();
    } else if selected_operation == 3 {
        // Add account password pair
        add_password();
    } else {
        // invalid option
        println!("Please select valid operation");
    }
}

fn main() -> Result<()> {
    let path = "./my_db.db3";
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users ( id INTEGER PRIMARY KEY, user_name TEXT NOT NULL UNIQUE, master_password TEXT NOT NULL)",
        (),
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

        let parsed_option: u8 = user_input.trim().parse().expect("Failed to parse integer");
        password_manager_operations(parsed_option, &conn);
        user_input.clear();
    }

    Ok(())
}
