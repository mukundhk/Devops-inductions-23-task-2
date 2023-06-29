ClockBuster:
A Netflix Clone using micro-service architecture<br />
Consists of multiple micro-services that have a singular function<br />
login-app-backend is the main REST API used to perform CRUD operations<br />

Tech Stack:
Language: Rust<br />
Framework: Actix-Web<br />
ORM: Diesel<br />
Database: PosgreSQL<br />

How to Setup Server[Linux installation]:<br />
- sudo apt install libpq-dev
- cargo install diesel_cli --no-default-features --features postgres
- CREATE DATABASE rust_server [in psql CLI];
- Add ENV variables for DB connection<br />
- diesel setup [Generates the tables]<br />
- cargo run [server runs]

Any problems with setting up server refer to official diesel documents