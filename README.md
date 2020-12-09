# locations

## developing locally

### server

```bash
> nix run .

# or if you want to develop on the server itself

# will create a nix shell with all required dependencies
> nix develop

# from launched nix shell
[nix-shell:/path/to/this/repo/server]$ cargo run
```

## Game Plan

Sasha:

- Hello World server in Rust

Nathan:

- Nix flake -> dev env -> container :shiny:

## Architecture

### Clients

iOS (Swift) and Android (Kotlin) native clients

Views:

- Map with points for each contact, with clickable contact list
- Contact list management - view/add/remove contacts
- Settings

### Server

Rust in a container on Azure

Operations:

- Upload my current location
- Get my contacts' locations
- Add a contact
- Remove a contact

## Links for the future

Mongo and Rust: https://devblogs.microsoft.com/cosmosdb/mongodb-and-rust/
Authentication in Rocket: https://medium.com/@james_32022/authentication-in-rocket-feb4f7223254
