# ¿How to use .env in Rust?

1. First Add the dependency dotenv in your Cargo.toml

```toml
[dependencies]
dotenv="*"
```
2. Load the .env file in your code

```rust
    dotenv::from_path("./.env").expect("error loading env");

```
3. Use the standard library for get your variables

```rust
let db_users = env::var("DB_USER").expect("env error");
    let db_password = env::var("DB_PASSWORD").expect("env error");
```