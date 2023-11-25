# README
## Prerequisite

install `diesel_cli` for use Diesel ORM

```bash
cargo install diesel_cli --no-default-features --features mysql
```

## Create a new Rust project

```bash
cargo new <project_name> --lib
cd <project_name>
```

## Create a database environment file

```bash
echo DATABASE_URL=mysql://root:root@localhost:3306/root > .env
# or
change `.env.test` to `.env`
```

## Generate initial migration

```bash
diesel setup
diesel migration generate <migration_name>
```

## Write the SQL for migration

```bash
# write the SQL for migration
```

## Migrate SQL

```bash
diesel migration run
```

## Check Cargo

```bash
cargo check
```
## How to use this application

```bash
cargo run --bin show_posts
cargo run --bin write_post
cargo run --bin publish_post
cargo run --bin delete_post
```