# Simple Rust HTTP Server

This is a basic HTTP server written in Rust to test some of the language fundamentals. It supports multithreading and maintains a shared global state.

## Prerequisites

1. **Install Rust and Cargo**:  
   Rust is required to compile and run this project. You can install Rust and Cargo using the following command:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   More information can be found in the [official Rust documentation](https://www.rust-lang.org/tools/install).

2. **Clone this repository**:
   ```bash
   git clone https://github.com/filipedanielski/rust-http-server.git
   cd rust-http-server
   ```

## How to Run the Server

1. Compile and run the server:
   ```bash
   cargo run
   ```
   The server will start on `127.0.0.1:8080`.

2. Access the server using an HTTP client (e.g., `curl` or a browser).

## Request Examples

### Request without parameters
Without providing `travel_to`, the server returns the current state:
```bash
curl "http://127.0.0.1:8080"
```
**Response**:
```
You are going to brazil
```

### Request with a valid `travel_to`
Send a valid country to update the state:
```bash
curl "http://127.0.0.1:8080/?travel_to=argentina"
```
**Response**:
```
You are going to argentina
```

### Request with an invalid `travel_to`
Send an invalid country to reset the state to `brazil`:
```bash
curl "http://127.0.0.1:8080/?travel_to=unknownland"
```
**Response**:
```
Country not found. You are going to brazil
```

![brazil.gif](./brazil.gif)

---

## Internal Functionality

1. **Valid States**:
   The server validates the `travel_to` parameter against a list of valid countries, including:
   ```
   brazil, argentina, usa, canada, france, germany, japan, china, india
   ```

2. **Global State**:
   - The initial state is `brazil`.
   - The state changes only when a valid country is provided in the `travel_to` parameter.

3. **Multithreading**:
   - Each connection is handled in a separate thread.
   - The global state is protected with a `Mutex` to prevent race conditions.

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Useful Links

- [Rust Installation](https://www.rust-lang.org/tools/install)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
