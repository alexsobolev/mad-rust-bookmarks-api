# Bookmarks API

A News Bookmarks API built with **Actix-web** and **MongoDB**. This project serves as a practical guide for Spring Boot developers transitioning to Rust, demonstrating how familiar patterns (Annotations, Dependency Injection, Middleware, Validation) translate to the Actix ecosystem.

This project implementation is based on the article: **[Mad Rust: Fury Road from Spring to Actix](https://sobolev.substack.com/p/mad-rust-2-fury-road-from-spring)**.

## 🚀 Features

- **Full CRUD**: Create, Read, Update, and Delete news bookmarks.
- **Advanced Filtering**: Search by tags and filter by unread status.
- **Pagination**: Built-in support for paginated results.
- **Robust Validation**: Request body validation using custom extractors and the `validator` crate.
- **Error Handling**: Centralized error management with custom `AppError` and `thiserror`.
- **Structured Logging**: Request tracing and logging using `tracing-actix-web`.
- **Environment Config**: Simple configuration via `.env` files.

## 🛠 Tech Stack

- **Framework**: [Actix Web 4](https://actix.rs/)
- **Database**: [MongoDB](https://www.mongodb.com/)
- **Serialization**: [Serde](https://serde.rs/)
- **Validation**: [Validator](https://github.com/Keats/validator)
- **Logging**: [Tracing](https://tracing.rs/)
- **Language**: Rust (Edition 2024)

## 📁 Project Structure

The project follows a modular structure that mirrors common Spring Boot architecture:

```text
src/
├── main.rs          # Entry point, server setup, and dependency wiring
├── state.rs         # Application state (DI container equivalent)
├── routes.rs        # Centralized route configuration
├── handlers/        # REST controllers / request handlers
├── service/         # Business logic layer
├── repository/      # Data access layer (MongoDB)
├── models/          # Data structures (DTOs and Entities)
├── extractors/      # Custom Actix extractors (e.g., ValidatedJson)
└── error.rs         # Error types and trait implementations
```

## 🏁 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)
- [Docker](https://www.docker.com/products/docker-desktop/) (for MongoDB)

### 1. Start MongoDB

Run a local MongoDB instance using Docker:

```bash
docker run -d -p 27017:27017 --name mongodb mongo:7
```

### 2. Configure Environment

Create a `.env` file in the root directory:

```env
MONGODB_URI=mongodb://localhost:27017
HOST=0.0.0.0
PORT=3000
RUST_LOG=info
```

### 3. Run the Application

```bash
cargo run
```

For development with hot-reloading, install `cargo-watch` first:

```bash
cargo install cargo-watch
cargo watch -x run
```

## 📡 API Endpoints

All endpoints are prefixed with `/api/bookmarks`.

| Method | Endpoint | Description | Query Parameters |
| :--- | :--- | :--- | :--- |
| `GET` | `/` | List all bookmarks | `tag`, `unread_only`, `page`, `size` |
| `GET` | `/{id}` | Get bookmark by ID | - |
| `POST` | `/` | Create a bookmark | - |
| `PUT` | `/{id}` | Update bookmark | - |
| `DELETE` | `/{id}` | Delete bookmark | - |

