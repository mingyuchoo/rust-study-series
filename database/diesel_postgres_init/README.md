# diesel_postgres_init

## Prerequisite

install `diesel_cli` for use Diesel ORM

```sh
cargo install diesel_cli
```

## Create a new Rust project

```sh
cargo new <project_name>
cd <project_name>
```

## Create a database environment file

```sh
echo DATABASE_URL=postgres://postgres:postgres@localhost/diesel_demo > .env

# or change `.env.test` to `.env`
```

## Generate initial migrations

```sh
diesel setup
diesel migration generate <migration_name>
```

## Write the SQL for migrations

```sh
# write the SQL for migrations
```

## Migrate SQL

```sh
diesel migration run
```
