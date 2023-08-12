# juniper_v014_mysql

GraphQL Implementation in Rust using Actix, Juniper, and Mysql as Database

## Prerequites

- install rustup
- install MySql

```bash
rustup default stable
rustup update stable
```

## Database Configuration

Create a new database for this project, and import the existing database schema has been provided named `01-init.sql, 02-ddl.sql, 03-dml.sql`.

Create `.env` file on the root directory of this project and set environment variable named `DATABASE_URL`, the example file has been provided named `.env.test`, you can see the format on there.

## How to build

```bash
cd juniper_v014_mysql
cargo build --profile dev     # for development
cargo build --profile release # for release
```

## How to run

```bash
docker-compose --file docker-compose.yml up --build --detach
cp .env.test .env
cargo run
```

## How to check the result, and test

Please visit <http://127.0.0.1:4000/graphql> and try it out.

```graphql
query {
  users {
    id
  }
}
```
