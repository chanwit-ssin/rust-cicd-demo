# rust-ping-pong

A simple ping-pong application written in Rust.

## CI/CD Flow

This project uses GitHub Actions for continuous integration and continuous deployment (CI/CD). 

- **main.yml**: This workflow runs tests and lints the code on every push to the `main` branch.
- **deploy.yml**: This workflow deploys the application to Railway on every push to the `main` branch.