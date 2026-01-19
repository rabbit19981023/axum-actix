# axum-actix

A Rust web application combining Axum web framework with Actix actor system.

## Overview

This project demonstrates integrating Actix actors with Axum for state management. A counter actor maintains state and handles messages for incrementing, decrementing, and retrieving the count.

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

Server will start at `http://0.0.0.0:3000`

## Endpoints

- `GET /` - Returns "Hello, world!"
- `GET /count` - Returns current count as JSON `{"count": <value>}`
- `GET /incr-count` - Increments the counter
- `GET /decr-count` - Decrements the counter
