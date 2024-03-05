# THIS JUSTFILE REQUIRES NUSHELL TO BE INSTALLED
set shell := ["nu", "-c"]

LOG_LEVEL := 'INFO'

default:
    @just --list

# build the program
build:
    cargo build

# run the program with bunyan tracing
bunyan level=LOG_LEVEL:
    RUST_LOG="INFO,signum_node_rs={{level}}" cargo run --features=bunyan | bunyan

# build the program for release
release:
    cargo build --release

# run the program
run level=LOG_LEVEL:
    RUST_LOG="INFO,signum_node_rs={{level}}" cargo run

# run cargo nextest
test:
    cargo watch -x "nextest run"
# cargo watch with default tracing (tracing-subscriber)
watch level=LOG_LEVEL:
    RUST_LOG="INFO,signum_node_rs={{level}}" cargo watch -x check -x "run"

# cargo watch with bunyan tracing
watch-bunyan level=LOG_LEVEL:
    RUST_LOG="INFO,signum_node_rs={{level}}" cargo watch -x check -x "run --features=bunyan | bunyan"

# reset the database
reset:
    DATABASE_URL="sqlite://signum.db3" sqlx database reset

# launch the app with support for tokio console (full recompile)
tokioconsole level=LOG_LEVEL:
    RUSTFLAGS="--cfg tokio_unstable" RUST_LOG="INFO,signum_node_rs={{level}}" cargo run --features=tokio-console
