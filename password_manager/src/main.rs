use rusqlite::{Connection, Result};
use std::io::stdin;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    master_password: String,
}

fn display_options_list() {
    const SCREEN_TEXT: [&str; 5] = [
        "PASSWORD MANAGER :",
        "1. Create Account",
        "2. Login",
        "3. Add Password",
        "Q. Quit",
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

fn find_user_by_username_and_password(
    conn: &Connection,
    username: &String,
    password: &String,
) -> Result<Option<User>> {
    let mut stmt = match conn.prepare(
        "SELECT id, user_name, master_password FROM users WHERE user_name=?1 AND master_password=?2",
    ) {
        Ok(stmt) => stmt,
        Err(err) => {
            println!("Error in prepare {}", err);
            return Err(err);
        }
    };
    println!("STMT after");
    let mut rows = stmt.query([&username, &password])?;

    println!("AFTER rows");

    if let Some(row) = rows.next()? {
        let user = User {
            id: row.get(0)?,
            name: row.get(1)?,
            master_password: row.get(2)?,
        };
        println!("User: {} {} {}", user.id, user.name, user.master_password);
        Ok(Some(user))
    } else {
        println!("login failed");
        Ok(None)
    }
}

fn login(conn: &Connection) -> Result<()> {
    let mut name = String::new();
    let mut master_password = String::new();

    println!("Enter username to login:");
    stdin()
        .read_line(&mut name)
        .expect("Failed to read username");
    name = name.trim().to_string();

    println!("Enter master password:");
    stdin()
        .read_line(&mut master_password)
        .expect("Failed to read master password");
    master_password = master_password.trim().to_string();

    let logged_user = find_user_by_username_and_password(&conn, &name, &master_password)?;

    println!("AFTER logged_user");

    match logged_user {
        Some(p) => println!("Logged in successfully\nWelcome {}", p.name),

        None => println!("Login Failed\nTry again with correct username and password"),
    }

    Ok(())
}

fn add_password(conn: &Connection, is_logged: bool, username: &String) -> Result<()> {
    if !is_logged {
        println!("Please login to add data")
    }

    let mut account_name = String::new();
    let mut account_password = String::new();

    println!("Add account name to store:");
    stdin()
        .read_line(&mut account_name)
        .expect("Failed to read account name");
    account_name = account_name.trim().to_string();

    println!("Add account password:");
    stdin()
        .read_line(&mut account_password)
        .expect("Failed to read account password");
    account_password = account_password.trim().to_string();

    Ok(())
}

fn password_manager_operations(selected_operation: u8, conn: &Connection) {
    if selected_operation == 1 {
        // Create Account
        let _ = create_account(&conn);
    } else if selected_operation == 2 {
        // Login
        let _ = login(&conn);
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

        if user_input.trim() == "q" || user_input.trim() == "Q" {
            break;
        }

        let parsed_option: u8 = user_input.trim().parse().expect("Failed to parse integer");
        password_manager_operations(parsed_option, &conn);
        user_input.clear();
    }

    Ok(())
}
