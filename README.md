# Ambient Messenger

**Ambient Messenger** is a cross-platform client-server messaging application developed in collaboration with the College of Business and Law (November 2023 – January 2024). The project focuses on performance, scalability, and multithreading support.

## 🛠️ Tech Stack

- **Server**:
  - Language: [Rust](https://www.rust-lang.org/)
  - Framework: [Actix Web](https://actix.rs/)
  - Multithreaded request handling for simultaneous clients
- **Client**:
  - [React](https://reactjs.org/)
  - [Electron](https://www.electronjs.org/) — for building the desktop application
- **Database**:
  - [Diesel](https://diesel.rs/) — ORM for Rust
- **Build & CI**:
  - GitHub Actions (see `.github/workflows/rust.yml`)

## ⚙️ Architecture

The project is structured as a modular multi-crate Rust application:
```grapql
AmbientMessenger
├── crates/
│   ├── adapter/           # API controllers and route logic
│   ├── adapters/          # Interfaces for API and database
│   ├── application/       # Use cases and domain interaction interfaces
│   ├── db/                # Database models and connections
│   ├── domain/            # Domain entities and business logic
│   └── infrastructure/    # Utility code and abstractions
└── src/
    └── main.rs            # Application entry point
```

## 🚀 Key Features

- **Multithreading**: the server handles multiple clients concurrently using parallel threads, maintaining high responsiveness under load.
- **Modular Architecture**: clean separation of concerns across domain, application, and infrastructure layers.
- **Database Migrations**: powered by Diesel.
- **CI/CD Support**: automated testing and builds via GitHub Actions.

## 📦 Getting Started

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository:
```bash
git clone https://github.com/your-username/ambient-messenger.git
cd ambient-messenger
```

3. Run database migrations:
```bash
cd crates/adapters
diesel migration run
```

4. Build and run the server:
```bash
cargo run --bin ambient-messenger
```

## 💻 Client
The client built with React and Electron communicates with the server via an HTTP API. You may need to navigate to a separate directory to build the client (not shown in the structure above — adjust as needed).

## 📄 License
This project is licensed under the MIT License.
