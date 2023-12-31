FROM rust:1-alpine3.19 as builder
WORKDIR /app
RUN apk add --no-cache musl-dev protobuf-dev
COPY Cargo.toml Cargo.lock build.rs ./
COPY .sqlx/ ./.sqlx/
COPY migrations/ ./migrations/
COPY proto/ ./proto/
COPY src/ ./src/
RUN cargo build --release --locked


FROM scratch as runtime
LABEL org.opencontainers.image.source="https://github.com/ScalabilityIssues/flight-manager"
COPY --from=builder /app/target/release/flight-mngr /app/flight-mngr
USER 65534:65534
ENTRYPOINT [ "/app/flight-mngr" ]