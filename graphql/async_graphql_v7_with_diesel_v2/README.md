# README
## How to use Diesel

```bash
cargo install diesel_cli --no-default-features --features postgres
echo DATABASE_URL=postgres://postgres:postgres@localhost/postgres > .env
diesel setup
diesel migration generate <migration_name>
```

## How to code format & lint

```bash
cargo fmt && cargo clippy --fix
```

## How to build & run

```bash
cargo run
```

### How to run as watch mod

```bash
cargo install cargo-watch
cargo watch -x 'run -- async_graphql_v4_with_diesel_v2'
```

## GraphQL Query for test

open `http://localhost:4000`

```graphql
query {
  members {
    id,
    name,
    teamId,
  }
}
```

## Reference

- https://async-graphql.github.io/async-graphql/en/quickstart.html
- https://github.com/async-graphql/examples
- https://github.com/async-graphql/async-graphql


