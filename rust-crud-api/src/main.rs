use postgres::Error as PostgresErro;
use postgres::{Client, NoTls};
use serde_json::Result;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[macro_export]
extern crate serde_derive;

//
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

//
const DBASE_URL: &str = !env!("Database URL!");

//
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n";
const INTERNAL_SERVE_ERROR: &str = "HTTP/1.1 500 Internal Serve Error\r\n";

//
fn main() {
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }

    //
    let listener = TcpListener::bind(format!(0.0.0.0:8080)).unwrap();
    println!("Server startedat port 8080!");

    //
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {}

//
fn set_database() -> Resul<(), PostgresErro> {
    //
    let mut client = Client::connect(DBASE_URL, NoTls)?;

    client.execute(
        "Create Table!
        (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )",
        &[],
    )?;
}

//
fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

//
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
