# [Dima Pristupa](https://dima.pristupa.dev/): test assignment for an adtech company

## About

- **Role:** Rust Back-End Engineer
- **Task:** [TaskWebServer.pdf](./assets/TaskWebServer.pdf)

## Quick start

```shell
# Run the application
cp .env.example .env 
docker compose -f ./docker/db.yml --env-file ./docker/.env.db  up
cargo run --release # or `just start-prod`

# Call the run endpoint
curl localhost:8080/api/v1/run -v # or `just call-run`

# Get all request group entities
curl -u "api_user:not_qwerty_password" localhost:8080/api/v1/request_group/ -v
# Get all request entities
curl -u "api_user:not_qwerty_password" localhost:8080/api/v1/request/ -v
```
**Tip:** you can use the admin panel to view tables via [http://localhost:9000](http://localhost:9000/?mysql=db&username=user&db=database&mysql=db&mysql=db)

```shell
# Run all tests (unit & integration)
cargo test --features integration_tests # or `just test`
```

## Local development

Additional dependencies:
- [mold](https://github.com/rui314/mold) 
- [cargo-watch](https://github.com/watchexec/cargo-watch)
- [just](https://github.com/casey/just) is used to run commands: 

```
$ just

🔧 Available commands:
   · start-prod         # Start the application (release build)
   · start              # Start the application (debug build)
   · start-watch        # Start the application (and watch for changes)
   · migrator_cli *ARGS # Run migrator_cli (check `just migrator_cli --help`)
   · test               # Run all tests (unit & integration)
   · test-watch         # Run all tests (and watch for changes)
   · format             # Format code
   · lint               # Run a linter
   · call-run           # Call /run endpoint

# Example
$ just start-watch
```

## Ideas for improvement

- Pagination for get all API requests
- Test coverage via [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin)
