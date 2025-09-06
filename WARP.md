# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Commonly Used Commands

- **Run in development mode**: `cargo run`
- **Run tests**: `cargo test`
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`
- **Build for release**: `cargo build --release`
- **Run database migrations**: `sqlx migrate run`

## High-Level Code Architecture and Structure

This project is a Rust-based web application that serves as a proof-of-concept for migrating from Spring Boot to Rust. It follows a layered architecture, which is organized as follows:

- **`main.rs`**: The application entry point.
- **`lib.rs`**: The library entry point, which exports all the modules.
- **`config.rs`**: Configuration management.
- **`errors.rs`**: Error handling.
- **`handlers.rs`**: API handlers.
- **`middleware.rs`**: Middleware for logging, CORS, etc.
- **`models.rs`**: Data models.
- **`repositories.rs`**: Data access layer.
- **`services.rs`**: Business logic layer.
- **`utils.rs`**: Utility functions.

The project has been updated to use the modern Rust module structure, which means that instead of using `mod.rs` files, each module has its own file with the module's name (e.g., `handlers.rs`).

## Getting Started

1.  **Install Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2.  **Install MySQL**: `brew install mysql` and `brew services start mysql`
3.  **Create the database**: `mysql -u root -p` and then `CREATE DATABASE rust_web_demo;`
4.  **Configure environment variables**: `cp env.example .env` and then edit the `.env` file.
5.  **Build the application**: `cargo build`
6.  **Run database migrations**: `sqlx migrate run`
7.  **Run the application**: `cargo run`

## API Endpoints

### Health Check

-   `GET /health`: Basic health check.
-   `GET /ready`: Readiness check (includes database connection check).

### Authentication

-   `POST /api/auth/register`: User registration.
-   `POST /api/auth/login`: User login.
-   `POST /api/auth/me`: Get current user information.

### User Management

-   `GET /api/users`: Get user list.
-   `POST /api/users`: Create user.
-   `GET /api/users/:id`: Get a specific user.
-   `PUT /api/users/:id`: Update a user.
-   `DELETE /api/users/:id`: Delete a user.

