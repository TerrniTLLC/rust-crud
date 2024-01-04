## To run the application stack

### 1.- First things first, copy environment variables from `.env.example`

```
$ cp .env.example .env
```

### 2.- actix-surrealdb-api, server/API built using Rust's Actix-web framework and SurrealDB database running in a Docker container.

In addition to the obvious prerequisite of having Rust and SurrealDB CLI installed, we need to do the following:

If we have _cargo-watch_ installed using:

```
$ cargo install cargo-watch
```

we won't have to restart the server every time we make a code change; running the following command in the root of the project actix-surrealdb-api:

```
$ cargo watch -x run
```

rather:

```
$ cargo run
```


Running database (SurrealDB)

```js
$ surreal start:NAME_OF_LOCAL_DB_FILE --user USER_SECRET --password PASSWORD_SECRET
```

the server will restart automatically 😀.
