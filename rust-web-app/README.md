# rust-web-app

## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:16

# (optional) To have a psql terminal got pg.
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

## DEV (watch)

> NOTE: Install cargo watch with `cargo install cargo-watch`

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run quick_dev.
cargo watch -q -c -w /examples -c "run --example quick_dev"
```

## Unit Test (watch)

```sh
cargo watch -q -c -w -x "test -- --capture"

# Specific test with filter.
cargo watch -q -c -w -x "test model::task::tests::test_create -- --nocapture"
```

## Dev

```sh
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo run --example qucik_dev
```

## Unit Test

```sh
cargo test -- --nocapture

cargo watch -q -c -x test model::task::tests::test_create -- --nocapture
```
