# async_graphql_v4_splitted_code

## How to use Diesel

```sh
cargo install diesel_cli --no-default-features --features postgres
echo DATABASE_URL=postgres://postgres:postgres@localhost/postgres > .env
diesel setup
diesel migration generate <migration_name>
```

## How to code format & lint

```sh
cargo fmt && cargo clippy --fix
```

## How to build & run

```sh
cargo run
```

### How to run as watch mod

```sh
cargo install cargo-watch
cargo watch -x 'run -- async_graphql_v4_with_diesel_v2'
```

## GraphQL Query for test

```graphql
query {
 add(a: 10, b: 20)
}
```

## Reference

- https://async-graphql.github.io/async-graphql/en/quickstart.html
- https://github.com/async-graphql/examples
- https://github.com/async-graphql/async-graphql


