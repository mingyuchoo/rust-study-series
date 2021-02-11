# GraphQL Server using Juniper

GraphQL Implementation in Rust using Actix, Juniper, and Mysql as Database

# Prerequites

- Rust Installed
- MySql as Database

# Database Configuration

Create a new database for this project, and import the existing database schema has been provided named `01-init.sql, 02-ddl.sql`.

Create `.env` file on the root directory of this project and set environment variable named `DATABASE_URL`, the example file has been provided named `.env.test`, you can see the format on there.

# Run

```sh
# go to the root dir
cd juniper-advanced

# Run
cargo run
```

### GraphQL Playground

http://127.0.0.1:4000/graphql
