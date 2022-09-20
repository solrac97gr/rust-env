use std::env;
extern crate dotenv;

fn main() {
    // Load .env file
    dotenv::from_path("./.env").expect("error loading env");

    // Get env variable
    let db_users = env::var("DB_USER").expect("env error");
    let db_password = env::var("DB_PASSWORD").expect("env error");

    // Print env variable
    println!("------------------------");
    println!("DB_USER: {}", db_users);
    println!("DB_PASSWORD: {}", db_password);
    println!("------------------------");

}
