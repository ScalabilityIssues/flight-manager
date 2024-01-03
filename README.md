# Flight Manager

## Development

### Requirements

- docker desktop
- rust: follow instruciton [here](https://www.rust-lang.org/tools/install)
- protobuf compiler: with the command `sudo apt install -y protobuf-compiler` Notice that the version should be >= 3.12
- sqlx-cli: with the command `cargo install sqlx-cli`

### Execution

1. `docker compose up -d`
1. execute the migrations with `cargo sqlx migrate run`
1. `cargo run`

**NOTICE**: The first and second commands should be executed only the first time
