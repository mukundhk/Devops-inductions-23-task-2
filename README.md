ClockBuster:
A Netflix Clone using micro-service architecture
Consists of multiple micro-services that have a singular function
login-app-backend is the main REST API used to perform CRUD operations 

Tech Stack:
Language: Rust
Framework: Actix-Web
ORM: Diesel
Database: PosgreSQL

How to Setup Server:
- cargo install diesel_cli
- cargo install diesel_cli --no-default-features --features postgres
- Add ENV variables for DB connection
Setup Schema:
- diesel setup
Start Server:
- cargo run


ToDo:
Add error handling for no users found
Add CI using github actions
Make a proper ReadMe for that poor joon
Add response when user deleted
Add basic login route

Any problems with setting up server refer to official diesel documents