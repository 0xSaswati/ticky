# Ticky

**A high-performance, asynchronous ticket management backend built in Rust**

---

## Table of Contents

* [Overview](#overview)
* [Features](#features)
* [Tech Stack](#tech-stack)
* [Architecture & Structure](#architecture--structure)
* [Setup & Installation](#setup--installation)
* [Running the Server](#running-the-server)
* [API Endpoints](#api-endpoints)
* [Database Migrations](#database-migrations)
* [Environment Variables](#environment-variables)
* [Testing](#testing)
* [Contributing](#contributing)
* [License](#license)

---

## Overview

Ticky is a lightweight ticket-management backend service designed for speed, safety, and maintainability. Written in Rust, it leverages async I/O and compile-time SQL validation to deliver a production-grade REST API for creating users, submitting tickets, and retrieving statistics.

---

## Features

* Create and manage users
* Submit and list support tickets
* Retrieve per-user ticket statistics (total, open, closed)
* Fully asynchronous, non-blocking I/O
* Compile-time checked SQL queries
* Version-controlled database schema migrations

---

## Tech Stack

| Layer           | Component  | Purpose                               |
| --------------- | ---------- | ------------------------------------- |
| Language        | Rust       | Memory safety, zero-cost abstractions |
| Web Framework   | Axum       | Type-safe routing, async handlers     |
| Async Runtime   | Tokio      | High-throughput non-blocking I/O      |
| Database Driver | SQLx       | Compile-time SQL checks, Rust mapping |
| Database Engine | PostgreSQL | ACID-compliant relational DB          |
| Migrations Tool | sqlx-cli   | Versioned SQL migrations              |
| Config Loader   | dotenvy    | Environment variable management       |
| Package Manager | Cargo      | Dependency management, builds         |

---

## Architecture & Structure

```
ticky-main/
├── migrations/         # Version-controlled SQL scripts
│   ├── 001_create_users.sql
│   └── 002_create_tickets.sql
├── src/                # Application source code
│   ├── db.rs           # Database pool initialization
│   ├── lib.rs          # Shared utilities (placeholder)
│   ├── main.rs         # Entry point & server bootstrap
│   ├── routes.rs       # Router configuration
│   ├── handlers/       # HTTP request handlers
│   │   ├── create_user.rs
│   │   ├── get_user.rs
│   │   ├── create_ticket.rs
│   │   ├── list_tickets.rs
│   │   └── user_stats.rs
│   └── models/         # Domain models and row mappings
│       ├── mod.rs
│       ├── user.rs
│       └── ticket.rs
├── .env                # Environment variables (excluded from Git)
├── Cargo.toml          # Project metadata & dependencies
├── Cargo.lock          # Locked dependency versions
├── .gitignore          # Ignored files and folders
└── README.md           # Project documentation
```

---

## Setup & Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/0xSaswati/ticky.git
   cd ticky-main
   ```

2. **Install Rust toolchain**

   * Follow instructions at [rustup.rs](https://rustup.rs)

3. **Install SQLx CLI**

   ```bash
   cargo install sqlx-cli --no-default-features --features postgres
   ```

4. **Create and configure the database**

   ```bash
   createdb ticky
   ```

5. **Configure environment**

   * Copy `.env.example` to `.env`
   * Set `DATABASE_URL=postgres://<user>:<password>@localhost/ticky`

---

## Running the Server

1. **Run database migrations**

   ```bash
   sqlx migrate run
   ```

2. **Start the application**

   ```bash
   cargo run
   ```

3. **Access the API**

   * Server listens on `http://localhost:8080`

---

## API Endpoints

| Method | Endpoint           | Description                        |
| ------ | ------------------ | ---------------------------------- |
| POST   | `/users`           | Create a new user                  |
| GET    | `/users/:id`       | Retrieve user details              |
| POST   | `/tickets`         | Create a new ticket                |
| GET    | `/tickets`         | List all tickets (optional filter) |
| GET    | `/users/:id/stats` | Get ticket stats for a user        |

**Request & Response Examples**

<details>
<summary>POST /users</summary>

**Request**

```json
{
  "name": "Alice",
  "email": "alice@example.com"
}
```

**Response**

```json
{
  "id": 1,
  "name": "Alice",
  "email": "alice@example.com"
}
```

</details>

<details>
<summary>GET /users/1/stats</summary>

**Response**

```json
{
  "total": 5,
  "open": 2,
  "closed": 3
}
```

</details>

---

## Database Migrations

Migrations are located in the `migrations/` folder and managed by **sqlx-cli**.

* **Run**: `sqlx migrate run`
* **Revert**: `sqlx migrate revert`

---

## Environment Variables

| Variable      | Description               |
| ------------- | ------------------------- |
| DATABASE\_URL | PostgreSQL connection URL |

---

## Testing

> *Currently, no automated tests included.*

You can manually test endpoints using tools like **Postman** or **curl**.

---

## Contributing

1. Fork the repo
2. Create a branch: `git checkout -b feature/YourFeature`
3. Commit changes: \`git commit -m "Add YourFeature"
4. Push branch: `git push origin feature/YourFeature`
5. Open a Pull Request
