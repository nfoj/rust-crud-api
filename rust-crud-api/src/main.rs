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
const DBASE_URL: &str = !env("Database URL!");

//
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 Internal Serve Error\r\n";

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

//
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if request_with("POST /users") => handle_post_request(r),
                r if request_with("GET /users/") => handle_get_request(r),
                r if request_with("GET /users") => handle_get_all_request(r),
                r if request_with("PUT users/") => handle_put_request(r),
                r if request_with("DELETE /users/") => handle_delete_request(r),
                _ => (NOT_FOUND, "Not Found", to_string()),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }

        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

//
fn handle_post_request(request: &str) -> (String, Striung) {
    match (
        get_user_request_body(&request),
        Client::connect(DBASE_URL, NoTls),
    ) {
        (Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "Insert Into users (name, email) VALEUES ($1, $2)",
                    &[&user.name, &user.email],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}


//
fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>, Client::connect(DBASE_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
        match client.query("SELECT * FROM users WHERE id = $1", &[&id]) {
        Ok(row) => {
                let user = User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                };

                (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
            }
            _ => (NOT_FOUND.to_string(), "User not found".to_string()),  
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "ERROR".to_string()),

    }
}

fn handle_get_all_request(request: &str) -> (String, String){
    match Client::connect(DBASE_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();
            
            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                   id: row.get(0),
                   name: row.get(1),
                   email: row.get(2), 
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }    
}

//
fn handle_put_request(request: &str) -> (String, String) {
        match 
        (
            get_id(&request).parse::<i32>,
            get_user_request_body(&request),
            Client::connect(DBASE_URL, NoTls),    
        )
    
    {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client 
                .execute(
                    "UPDATE user SET name = $1, email = $2, WHERE id = $3",
                    &[&user.name, &user.email, &id]
                )
                .unwrap()

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
    
}

//
fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>, Client::connect(DBASE_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap()

            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "USER not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        } 

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}



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
