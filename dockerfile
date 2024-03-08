FROM rust:1-alpine3.19 AS chef
RUN apk add --no-cache musl-dev protobuf-dev
RUN cargo install cargo-chef
WORKDIR /app


FROM chef AS planner
COPY Cargo.toml Cargo.lock build.rs ./
COPY src/ ./src/
RUN cargo chef prepare  --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock build.rs ./
COPY .sqlx/ ./.sqlx/
COPY migrations/ ./migrations/
COPY proto/ ./proto/
COPY src/ ./src/
RUN cargo build --release --locked


FROM scratch AS runtime
LABEL org.opencontainers.image.source="https://github.com/ScalabilityIssues/flight-manager"
COPY --from=builder /app/target/release/flightmngr /app/flightmngr
USER 65534:65534
ENTRYPOINT [ "/app/flightmngr" ]