# server

## Developing locally

```bash
# this requires Nix & direnv (you can use it without the global direnv hook
# for manual loading of the env)
# will make dependendencies specfied by flake.nix
# available in your local env
> source load_env
> cargo run
```

## Architecture

Rust in a container on Azure

Operations:

- Upload my current location
- Get my contacts' locations
- Add a contact
- Remove a contact

## Links for the future

Mongo and Rust: https://devblogs.microsoft.com/cosmosdb/mongodb-and-rust/
Authentication in Rocket: https://medium.com/@james_32022/authentication-in-rocket-feb4f7223254
