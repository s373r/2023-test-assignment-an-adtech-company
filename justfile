set dotenv-load

[private]
default:
    @just --list --unsorted --list-heading $'🔧 Available commands:\n' --list-prefix '   · '

# Start the application (release build)
start-prod:
    mold -run cargo run --release

# Start the application (debug build)
start:
    mold -run cargo run

# Start the application (and watch for changes)
start-watch:
    mold -run cargo watch -x run

# Run migrator_cli (check `just migrator_cli --help`)
migrator_cli *ARGS:
    mold -run cargo run --manifest-path ./migration/Cargo.toml -- {{ARGS}}

# Run all tests (unit & integration)
test:
    mold -run cargo test --features integration_tests -- --nocapture

# Run all tests (and watch for changes)
test-watch:
    mold -run cargo watch -x 'test --features integration_tests -- --nocapture'

# Format code
format:
    cargo fmt --all -v

# Run a linter
lint:
    mold -run cargo clippy --all-targets --all-features --all --fix --allow-dirty --allow-staged -- -D clippy::all

# Call /run endpoint
call-run:
    curl localhost:8080/api/v1/run -v
