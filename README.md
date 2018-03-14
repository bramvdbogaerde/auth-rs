Auth-rs
=============

[![](https://img.shields.io/badge/crates.io-v0.4.0-red.svg)](https://crates.io/crates/rocket-simpleauth)

This library provides a simple username/password authentication system to use with Rocket.

For Cookie encryption, the library uses the Private Cookie feature of Rocket. For maintaining cookie validity after a restart,
do not forget to set the `secret_key` configuration parameter, otherwise Rocket will generate a new key at every execution.

Cargo.toml
```toml
[dependencies]
rocket = "0.3.0"
rocket-simpleauth = "0.4.0"
rocket_codegen = "0.3.0"
```

## Example

Please check [example/](example/) directory, for a full example. 

## API Stability

Apart from a few functions of some traits, the API should stay the same from now on (v0.4.0).
Given issue #6 it is probabel that the `Authenticator` trait will change.

When this crate reaches 1.0.0, the full API will be considered stable and frozen.

## Todo

The following items are in development or are planned to be developped:

* [ ] standard implementation for user storage into sqlite databases
* [x] publishing to crates.io
* [ ] Documentation

### On hold

See [this issue](https://github.com/bramvdbogaerde/auth-rs/issues/4)

* [ ] cookie storage in Redis datastore
* [ ] stateless cookie validation using JSON Web Tokens


