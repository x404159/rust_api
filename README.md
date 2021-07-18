# About

this is a **REST API** in **_rust_** to manage users.
it uses:

- postgresql as database with [diesel](https://diesel.rs)
- [actix-web](https://actix.rs/) as server
- [rust-argon2](https://crates.io/crates/rust-argon2) for hashing password
- r2d2 to create database connection pool

## features

- password hashing with rust-argon2
- authentication with cookies (not the best) using actix-identity
- only admin user can convert normal user to admin //not implemented yet
- only admin can view all users
