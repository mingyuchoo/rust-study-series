# README
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
cargo watch -x 'run -- async_graphql_v4'
```

## GraphQL Query for test

open `http://localhost:4000`

```graphql
query {
 add(a: 10, b: 20)
}
```

## Reference

- https://async-graphql.github.io/async-graphql/en/quickstart.html
- https://github.com/async-graphql/examples
- https://github.com/async-graphql/async-graphql
