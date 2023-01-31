# syntax=docker/dockerfile:1

# AMD64
FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:x86_64-musl as builder-amd64

# ARM64
FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:aarch64-musl as builder-arm64

ARG TARGETARCH
FROM builder-$TARGETARCH as builder

RUN apt update && apt install -y protobuf-compiler

RUN adduser --disabled-password --disabled-login --gecos "" --no-create-home gen-axum

RUN cargo init

# touch lib.rs as we combine both
RUN touch src/lib.rs

# copy cargo.*
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

# cache depencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
RUN --mount=type=cache,target=$CARGO_HOME/registry \
    --mount=type=cache,target=$CARGO_HOME/.git \
    --mount=type=cache,target=gen-axum/target,sharing=locked \
    cargo build --target $CARGO_BUILD_TARGET --bin gen-axum-app --release

# copy src
COPY src ./src
# copy config
COPY config ./config

# final build for release
RUN rm ./target/$CARGO_BUILD_TARGET/release/deps/*gen_axum*
RUN --mount=type=cache,target=$CARGO_HOME/registry \
    --mount=type=cache,target=$CARGO_HOME/.git \
    --mount=type=cache,target=gen-axum/target,sharing=locked \
    cargo build --target $CARGO_BUILD_TARGET --bin gen-axum-app --release

RUN musl-strip ./target/$CARGO_BUILD_TARGET/release/gen-axum-app

RUN mv ./target/$CARGO_BUILD_TARGET/release/gen-axum* /usr/local/bin
RUN mv ./config /etc/config

FROM scratch

ARG backtrace=0
ARG log_level=info

ENV RUST_BACKTRACE=${backtrace} \
    RUST_LOG=${log_level}

COPY --from=builder /usr/local/bin/gen-axum* .
COPY --from=builder /etc/config ./config
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER gen-axum:gen-axum

EXPOSE 3000
EXPOSE 4000
ENTRYPOINT ["./gen-axum-app"]
