# light-letter

## Installation

1. Install rust toolchain installer [website](https://rustup.rs/)
1. Install latest rust stable `rustup install stable`
1. Install PostgreSQL `apt install postgresql libpq-dev` (debian like systems)
1. Login into PostgreSQL `sudo -u postgres psql`
  1. Create a new user `CREATE USER light_letter;`
  1. Allow new user to create database `ALTER USER light_letter CREATEDB;`
  1. `ALTER USER light_letter WITH PASSWORD 'your_password'`;
  1. `\q`
1. Clone this repo
1. Create a `config.toml` from examples
1. Run it with `cargo run`
