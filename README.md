# Notify Service

A Rust-based notification service built with Axum framework.

## Features

- RESTful API endpoints for notification management
- Support for multiple notification channels
- Built with Rust and Axum for high performance
- Containerized deployment ready

## Prerequisites

- Rust 1.73 or higher
- Docker (for containerized deployment)
- Make (optional, for using Makefile commands)

## Local Development

1. Clone the repository
```bash
git clone https://github.com/yourusername/notify.git
cd notify
```

2. Install dependencies
```bash
cargo build
```

3. Run the service
```bash
cargo run
```

The service will be available at `http://localhost:9001`

## Docker Deployment

### Build the Image

```bash
docker build -t prongbang/notify:latest -f deployments/Dockerfile .
```

### Run the Container

Basic run:
```bash
docker run -d -p 9001:9001 prongbang/notify:latest
```

With environment variables:
```bash
docker run \
	-e SERVER_HOST="0.0.0.0" \
	-e SERVER_PORT="9001" \
	-e BUDDHA_ENDPOINT="http://buddha.com" \
	-e DISCORD_WEBHOOK_URL="http://discord.com" \
	-e API_KEY="XYZ" \
	-it -p 9001:9001 prongbang/notify:latest
```

### Docker Commands

View logs:
```bash
docker logs notify-service
docker logs -f notify-service  # Follow logs
```

Stop container:
```bash
docker stop notify-service
```

Remove container:
```bash
docker rm notify-service
```
