Spider Devops Inductions - Task 2 <br />

<br />Instructions:<br />
Fork the given repo and set the your repo to private<br />
Setup the server using the instructions given below<br />
Dockerize the given application using Docker and Docker-compose<br />

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

Any problems with setting up server refer to official cargo and diesel documents