# Actix web + Diesel + Yew SSR

Basic complete web application with rust.

## Usage

### Install SQLite

```sh
# on OpenSUSE
sudo zypper install sqlite3-devel libsqlite3-0 sqlite3

# on Ubuntu
sudo apt-get install libsqlite3-dev sqlite3

# on Fedora
sudo dnf install libsqlite3x-devel sqlite3x

# on macOS (using homebrew)
brew install sqlite3
```

### Initialize SQLite Database

```sh

# UNIX
cargo install diesel_cli --no-default-features --features sqlite

# IF NOT WORK USE THIS

cargo install diesel_cli --no-default-features --features sqlite-bundled

echo "DATABASE_URL=db/test.db" > .env
diesel migration run
```

There will now be a database file at `./test.db`.

### Running Server

```sh
cargo run

# Started http server: 127.0.0.1:8080
```

### Explore The SQLite DB

```sh
sqlite3 test.db
```

```
sqlite> .tables
sqlite> SELECT * FROM users;
```

## Using Other Databases

You can find a complete example of Diesel + PostgreSQL at: [https://github.com/TechEmpower/FrameworkBenchmarks/tree/master/frameworks/Rust/actix](https://github.com/TechEmpower/FrameworkBenchmarks/tree/master/frameworks/Rust/actix)