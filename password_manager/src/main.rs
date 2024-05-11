use std::io::stdin;

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

fn main() {
    let mut user_input = String::new();
    loop {
        display_options_list();
        stdin()
            .read_line(&mut user_input)
            .expect("Error while reading line");
        println!("{}", user_input.trim());
        user_input.clear();
    }
}
