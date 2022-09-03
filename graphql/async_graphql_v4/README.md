# async_graphql_v4

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
cargo watch -x 'run -- async_graphql_v4'
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


