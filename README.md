Auth-rs
=============

[![](https://img.shields.io/badge/crates.io-v0.3.0-red.svg)](https://crates.io/crates/rocket-simpleauth)

This library provides a simple username/password authentication system to use with Rocket.
At the moment the library is not yet published at crates.io, but can be used via this Git repository.

For Cookie encryption, the library uses the Private Cookie feature of Rocket. For maintaining cookie validity after a restart,
do not forget to set the `secret_key` configuration parameter, otherwise Rocket will generate a new key at every execution.

Cargo.toml
```toml
[dependencies]
rocket = "0.3.0"
rocket-simpleauth = "0.3.0"
rocket_codegen = "0.3.0"
```

## Example

Please check [example/](example/) directory, for a full example. 

## Todo

The following items are in development or are planned to be developped:

* [ ] standard implementation for user storage into sqlite databases
* [x] publishing to crates.io
* [ ] API stability
* [ ] Documentation

### On hold

See [this issue](https://github.com/bramvdbogaerde/auth-rs/issues/4)

* [ ] cookie storage in Redis datastore
* [ ] stateless cookie validation using JSON Web Tokens


