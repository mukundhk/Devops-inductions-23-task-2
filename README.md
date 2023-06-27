ClockBuster:
A Netflix Clone using micro-service architecture<br />
Consists of multiple micro-services that have a singular function<br />
login-app-backend is the main REST API used to perform CRUD operations<br />

Tech Stack:
Language: Rust<br />
Framework: Actix-Web<br />
ORM: Diesel<br />
Database: PosgreSQL<br />

How to Setup Server:<br />
- cargo install diesel_cli
- cargo install diesel_cli --no-default-features --features postgres
- Add ENV variables for DB connection<br />
Setup Schema:<br />
- diesel setup<br />
Start Server:<br />
- cargo run

Any problems with setting up server refer to official diesel documents