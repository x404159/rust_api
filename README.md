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
- only admin user can change a normal user account to admin account
- only admin can view all users (only emails)
- users can delete there own account
- users can change there own email, password and name
- auto logout after deletion of account or attempting to change password

### added

- some tests as an example of tests with actix-web

#### note

this is my first work in actix-web and diesel on this scale.
so i don't know much of the best practices.
