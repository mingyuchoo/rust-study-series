# diesel_mysql_init

## Prerequisite

install `diesel_cli` for use Diesel ORM

```bash
cargo install diesel_cli
```

## Create a new Rust project

```bash
cargo new <project_name>
cd <project_name>
```

## Create a database environment file

```bash
echo DATABASE_URL=postgresql://postgres:postgres@localhost:5432/diesel_mysql_init > .env

# or change `.env.test` to `.env`
```

## Generate initial migrations

```bash
diesel setup
diesel migration generate <migration_name>
```

## Write the SQL for migrations

```bash
# write the SQL for migrations
```

## Migrate SQL

```bash
diesel migration run
```
