# Sonar server

Sonar server is a Rust project built on [Rocket](https://github.com/SergioBenitez/Rocket),
with authentication via OpenID Connect.

## Developing

### Rust

Sonar uses the `nightly` Rust toolchain, and requires at least:

```sh
> rustc --version
rustc 1.50.0-nightly (7efc097c4 2020-12-12)
```

To check, run, or test, ensure you have `rustc` available as above and run:

```sh
> cargo check # Include --all-targets to also check the tests
> cargo run
> cargo test
```

### Mongo

Sonar uses [MongoDB](https://docs.mongodb.com) as its database. To run, first
make sure you have a Mongo instance running and that the correct connection
string (e.g., `"mongodb://localhost:27017"`is passed during startup.

If you installed Mongo via Homebrew, you can run:

```sh
$ mongod --config </usr/local/etc/mongod.conf | path-to-config>
```

to start a Mongo instance. The config specifies, for example, the binding IP
(e.g., `localhost`) and port (defaults to `27017`).

You can also use the Mongo shell to inspect or ad-hoc modify the Mongo store
during development, e.g.:

```sh
$ mongo
> show dbs
sonar ...
> use sonar
switch to db sonar
> show collections
users
> db.users.find()
# prints the contents of the `users` collection
> db.users.drop()
# delete the `users` collection
```

### Nix build environment

Sonar also has (will eventually have) a build environment managed by a Nix
flake. You can run

```sh
> nix develop
```

to load a Bash shell with the appropriate dependencies (e.g., `rustc`), and then
run the build commands above. If you have `direnv` available, run:

```sh
> touch .use-flake
```

to opt into `direnv` integration with the flake.

## Design

### Operations

- Upload my current location
- Add a contact
- Remove a contact
- Get my contacts' locations

### Data modeling

User:

```json
{
  "id": GUID,
  "last_location": Location,
  "last_update": Timestamp,
  "shared_to": ListOfGuid,
  "shared_with_me_hint": ListOfGuid
}
```
