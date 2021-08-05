# About

this is a **REST API** in **_rust_** to manage users.
it uses:

- postgresql as database with [diesel](https://diesel.rs)
- [actix-web](https://actix.rs/) as server
- [rust-argon2](https://crates.io/crates/rust-argon2) for hashing password
- r2d2 to create database connection pool
- jsonwebtoken for authentication

## features

- password hashing with rust-argon2
- ~~authentication with cookies (not the best) using actix-identity~~ replaced with jsonwebtoken auth
- only admin user can change a normal user account to admin account
- only admin can view all users (only emails)
- users can delete there own account
- users can change there own email, password and name
- auto logout after deletion of account or attempting to change password

### routes

| Method | Route       | body                      | Success Response                  | Description                                    |
| ------ | ----------- | ------------------------- | --------------------------------- | ---------------------------------------------- |
| POST   | /users      | `{name, email, password}` | `{ email }`                       | creation of user / register                    |
| GET    | /users      | N/A                       | `{email, clearance}`              | get all the users (only for admins)            |
| PATCH  | /users/{id} | `{ update_value_only }`   | `{email, name, password: hidden}` | update user with id (only for admins)          |
| POST   | /auth       | `{ email, password }`     | `{ token }`                       | login                                          |
| DELETE | /auth       | N/A                       | Statuscode 200                    | logout (sets the authorization token to blank) |
| GET    | /user       | N/A                       | `{user_details}`                  | get logged user                                |
| PATCH  | /user       | `{ update_value_only }`   | `{email, name, password: hidden}` | update logged user                             |
| DELETE | /user       | N/A                       | `{ msg }`                         | delete logged user from database               |
| GET    | /user/{id}  | N/A                       | `{email, id}`                     | get user by id (only for authorized users)     |

#### added

- some tests as an example of tests with actix-web

#### note

this is my first work in actix-web and diesel on this scale.
so i don't know much of the best practices.
