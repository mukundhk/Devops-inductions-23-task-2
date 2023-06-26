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
- Add ENV variables for DB connection
Setup Schema:<br />
- diesel setup
Start Server:<br />
- cargo run

ToDo:
- Add error handling for no users found
- Make a proper ReadMe for that poor joon
- Add response when user deleted
- Add basic login route

Any problems with setting up server refer to official diesel documents