# Magic Voice Backend

## Project Structure

```
magic_voice_be/
├── src/
│   ├── main.rs         # Entrypoint, minimal Axum server
│   ├── api/            # API layer
│   │   └── mod.rs
│   ├── domain/         # Domain logic
│   │   └── mod.rs
│   ├── services/       # Service layer
│   │   └── mod.rs
│   ├── db/             # Database layer
│   │   └── mod.rs
│   └── config/         # Configuration
│       └── mod.rs
├── Cargo.toml          # Dependencies and metadata
└── README.md           # Project documentation
```

## How to Run

1. Install Rust (https://rustup.rs/)
2. Run the server:
   ```sh
   cargo run
   ```
3. The server will start on http://127.0.0.1:3000

## Dependencies
- axum
- tokio
- tower

## Description
This project is a minimal Axum-based Rust backend with a clean modular structure, ready for further development.

