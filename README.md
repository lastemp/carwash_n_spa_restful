# carwash_n_spa_restful

This RESTful Actix Web API that integrates with mobile app and is meant to enable Carwash owners to automate their record keeping and payments.

The RESTful Actix Web API has below listed dependencies:
- [Actix Web](https://github.com/actix/actix-web) web framework for Rust
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [base64](https://github.com/marshallpierce/rust-base64) Decode from Base64 format or encode into it
- [mysql](https://github.com/blackbeam/rust-mysql-simple) MySql database driver
- [dotenv](https://github.com/dotenv-rs/dotenv) Loads environment variables from a .env file
- [MySQL](https://github.com/mysql/mysql-server) MySQL database server

You'll need to have a MySQL (or compatible) server running on your machine to test this example.

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../carwash_n_spa_restful
```

1. Create database, tables and stored-procedures:

   The `sql` directory contains the SQL files used for database setup:
   
   Database
   ```sh
   mysql -u root -p < sql/0_create_database.sql
   ```
   
   Tables
   ```sh
   mysql -u root -p carwash_n_spa < sql/tables/*.sql
   ```
   
   Stored procedures
   ```sh
   mysql -u root -p carwash_n_spa < sql/stored-procedures/*.sql
   ```

   For each step you will be prompted for the root user's password. If there's no password set on the root use, just hit enter again.
   
   NB: The Database tables and stored-procedures have not been uploaded!

1. Create a `.env` file in this this directory:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   MYSQL_USER=root
   MYSQL_PASSWORD=<password>
   MYSQL_HOST=127.0.0.1
   MYSQL_PORT=3306
   MYSQL_DBNAME=carwash_n_spa
   ```

   Update "MYSQL_USER" and "MYSQL_PASSWORD" values with the correct MySQL user/password.

1. Run the server:

   ```sh
   cargo run
   ```

1. Using a different terminal send requests to the running server. For example, using [HTTPie]:

   ```sh
   http POST :8080/getsearchsalesdata "search_by"={"customer_name": true} "search_data"="john"
   ```

   See [the API documentation pages](./apis/) for more info.

[HTTPie]: https://httpie.io/cli
