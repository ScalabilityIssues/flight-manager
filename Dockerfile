ARG RUST_VERSION=1.76.0
ARG APP_NAME=flightmngr


FROM rust:${RUST_VERSION}-alpine AS chef
ARG APP_NAME
RUN apk add --no-cache musl-dev protobuf-dev
RUN cargo install cargo-chef
WORKDIR /app


FROM chef AS planner
COPY . ./
RUN cargo chef prepare  --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . ./
RUN cargo build --release --locked


FROM scratch AS runtime
LABEL org.opencontainers.image.source="https://github.com/ScalabilityIssues/flight-manager"
COPY --from=builder /app/target/release/flightmngr /app/flightmngr
USER 65534:65534
ENV RUST_LOG=info
EXPOSE 80
ENTRYPOINT [ "/app/flightmngr" ]