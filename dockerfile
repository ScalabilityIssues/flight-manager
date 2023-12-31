FROM rust:1-alpine3.19 as chef
WORKDIR /app
RUN apk add --no-cache musl-dev protobuf-dev
RUN cargo install cargo-chef --locked


FROM chef as planner
COPY Cargo.toml Cargo.lock build.rs ./
COPY src/ ./src/
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY .sqlx/ ./.sqlx/
COPY migrations/ ./migrations/
COPY proto/ ./proto/
COPY src/ ./src/
COPY build.rs ./

RUN cargo build --release


FROM scratch as runtime
COPY --from=builder /app/target/release/flight-mngr /app/flight-mngr
USER 65534:65534
ENTRYPOINT [ "/app/flight-mngr" ]