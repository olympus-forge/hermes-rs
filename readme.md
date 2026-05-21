# 🪽 hermes-rs

A lightweight, production-ready **email delivery service** built with Rust. Hermes-rs exposes an HTTP API for sending emails, designed to be fast, reliable, and easy to deploy via Docker.

---

## ✨ Features

- ⚡ High-performance async HTTP server (Axum / Tokio)
- 📧 Email sending via configurable provider
- 🔧 TOML-based configuration
- 🐳 Docker & Docker Compose support
- 🩺 Health check endpoint
- 🛑 Graceful shutdown handling
- 📋 Structured logging

---

## 📁 Project Structure

```
hermes-rs/
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
├── docker-compose.yml
├── configs/
│   └── default.toml          # Default configuration file
└── src/
    ├── main.rs               # Entry point
    ├── lib.rs                # Library root
    ├── bootstrap/            # App initialization
    │   ├── app.rs            # Application setup
    │   ├── logger.rs         # Logging initialization
    │   ├── router.rs         # Route registration
    │   ├── server.rs         # HTTP server setup
    │   ├── shutdown.rs       # Graceful shutdown signal handler
    │   └── state.rs          # Shared application state
    ├── config/               # Configuration loading
    │   └── settings.rs       # Settings struct & deserialization
    ├── domain/               # Core domain models
    ├── errors/               # Unified error types
    ├── handlers/             # HTTP request handlers
    │   ├── email.rs          # Email send endpoint
    │   └── health.rs         # Health check endpoint
    ├── providers/            # Email provider integrations
    ├── repository/           # Data access layer
    └── services/
        └── email_service.rs  # Email business logic
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) >= 1.75
- [Docker](https://www.docker.com/) (optional)

### Running Locally

```bash
# Clone the repository
git clone https://github.com/olympus-forge/hermes-rs.git
cd hermes-rs

# Build the project
cargo build --release

# Run the service
cargo run
```

The service will start using the configuration at `configs/default.toml`.

### Running with Docker

```bash
# Build and start the service
docker-compose up --build

# Run in detached mode
docker-compose up -d --build
```

---

## ⚙️ Configuration

Configuration is managed via `configs/default.toml`. You can override values using environment variables.

```toml
# configs/default.toml (example)

[app]
app_name = "Hermes Core"
app_host = "0.0.0.0"
app_port = 8080
app_version = "0.1.0"
env = "development"
debug = true

[http]
request_timeout_secs = 15
max_body_size_mb = 5

[log]
log_level = "info"
log_format = "json"

[email]
provider = "smtp"
from_name = "Hermes Core"
from_email = "noreply@example.com"

[email.smtp]
host = "mailpit"
port = 1025
username = ""
password = ""
pool_size = 5
starttls = true

[retry]
max_attempts = 3
backoff_secs = 10
worker_interval_secs = 30

[rate_limit]
enabled = false
requests_per_minute = 60
```

> ⚠️ Never commit credentials to version control. Use environment variables or a secrets manager in production.

---

## 📡 API Endpoints

### `GET /health-check`

Returns the health status of the service.

**Response:**
```json
{
  "status": "ok"
}
```

---

### `POST /email/send`

Sends an email.

**Request Body:**
```json
{
  "to": "recipient@example.com",
  "subject": "Hello from Hermes",
  "body": "This is the email body."
}
```

**Response:**
```json
{
  "message": "Email sent successfully"
}
```

---

## 🐳 Docker

The included `Dockerfile` uses a multi-stage build to produce a minimal production image.

```bash
# Build the image manually
docker build -t hermes-rs .

# Run the container
docker run -p 8080:8080 hermes-rs
```

---

## 🧪 Running Tests

```bash
cargo test
```

---

## 📦 Tech Stack

| Component     | Technology          |
|---------------|---------------------|
| Language      | Rust                |
| HTTP Framework| Axum                |
| Async Runtime | Tokio               |
| Configuration | TOML + config crate |
| Logging       | tracing             |
| Containerization | Docker / Compose |

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

> **Hermes** — messenger of the gods, now delivering your emails. ⚡