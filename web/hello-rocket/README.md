# README

## Prerequisites

### Install Rust

Install rust with rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Change default rust compiler to nightly

```bash
rustup default nightly
```

### Install Linux Libraris

If you are using Ubuntu Linux

```bash
sudo apt install -y libpq-dev          # for the PostgreSQL
sudo apt install -y libmysqlclient-dev # for the MySQL
sudo apt install -y libsqlite3-dev     # for the SQLite
```

If you are using Fedora Linux

```bash
sudo dnf install -y postgresql-devel
```

If you are using macOS

```bash
brew install postgresql
cd $HOME/.cargo
touch config.toml
```

Add these content to `config.toml`

```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

## Install Diesel CLI for PostgreSQL

Install `diesel_cli` for use Diesel ORM

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## Create a new Rust project

```bash
cargo new <project_name> --bin
cd <project_name>
```

## Create a database environment file

```bash
echo "export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres" > .envrc
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
# write the SQL for migrtions
```

## Migrate SQL

```bash
diesel migration run
```

## Run and access to the endpoint

```bash
cargo check
cargo run
```

Access to the endpoint with Web browsers or other tools

```
- GET  /api/health
- POST /api/posts
- GET  /api/posts?<page>&<list>
```
## Tips

### How to add dependencies

```bash
cargo add <dependency_name> --features <feature_name> <feature_name>
### Run as watch mode

```bash
cargo install cargo-watch
cargo watch -x run
```

```bash
curl -X GET http://localhost:8000/api/posts?page=1&limit=10
```
