use httparse;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub trait Controller {
    fn index(&self, state: &str) -> String;
    fn error_response(&self) -> String;
}

pub struct TravelController;

impl Controller for TravelController {
    fn index(&self, state: &str) -> String {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nYou are going to {}\n",
            state
        )
    }

    fn error_response(&self) -> String {
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nCountry not found. You are going to brazil\n".to_string()
    }
}

pub struct Server {
    address: String,
    state: Arc<Mutex<String>>, // global state
    valid_countries: Vec<String>,
}

impl Server {
    pub fn new(address: &str, valid_countries: Vec<&str>) -> Self {
        Server {
            address: address.to_string(),
            state: Arc::new(Mutex::new(String::from("brazil"))), // initial state 'brazil'
            valid_countries: valid_countries.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn start<C: Controller + Send + Sync + 'static>(self, controller: C) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind address");
        let state = self.state.clone();
        let valid_countries = self.valid_countries.clone();

        println!("Server running on {}", self.address);

        let value = Arc::new(controller);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let state = state.clone();
                    let controller = value.clone();
                    let valid_countries = valid_countries.clone();

                    thread::spawn(move || {
                        handle_connection(stream, controller, state, &valid_countries);
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }
}

fn handle_connection<C: Controller>(
    mut stream: TcpStream,
    controller: Arc<C>,
    state: Arc<Mutex<String>>,
    valid_countries: &[String],
) {
    println!("Thread started for connection");
    let mut buffer = [0; 512];
    if let Err(_) = stream.read(&mut buffer) {
        return;
    }

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut request = httparse::Request::new(&mut headers);

    if request.parse(&buffer).is_err() {
        return;
    }

    let path = request.path.unwrap_or("/");
    let mut updated_state = {
        let st = state.lock().unwrap();
        st.clone()
    };

    let mut valid_request = true;

    // checks for a query string
    if let Some(idx) = path.find('?') {
        let query_str = &path[idx + 1..]; // everything after '?'
        if let Some(value_idx) = query_str.find("travel_to=") {
            let travel_value = &query_str[value_idx + "travel_to=".len()..];
            if valid_countries.contains(&travel_value.to_string()) {
                let mut st = state.lock().unwrap();
                *st = travel_value.to_string();
                updated_state = travel_value.to_string();
            } else {
                let mut st = state.lock().unwrap();
                *st = "brazil".to_string();
                updated_state = "brazil".to_string();
                valid_request = false;
            }
        }
    }

    let response = if valid_request {
        controller.index(&updated_state)
    } else {
        //you tried travelling to an invalid place, so now you are going to brazil
        controller.error_response()
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let valid_countries = vec![
        "brazil",
        "argentina",
        "usa",
        "canada",
        "france",
        "germany",
        "japan",
        "china",
        "india",
    ];

    let server = Server::new("127.0.0.1:8080", valid_countries);
    let controller = TravelController;

    server.start(controller);
}
