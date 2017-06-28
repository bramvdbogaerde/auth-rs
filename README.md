Auth-rs
=============

[![](https://img.shields.io/badge/crates.io-v0.1.0-red.svg)](https://crates.io/crates/rocket-simpleauth)

This library provides a simple username/password authentication system to use with Rocket.
At the moment the library is not yet published at crates.io, but can be used via this Git repository.

## Example

In this example we are using the builtin `DummyAuthenticator` which lets all username/password combinations through.
As the cookie content a simple "Hello World" string is used.

SimpleCookie just stores the cookie in plain text without any encoding or encryption.
This is just an example, please implement your own Authenticator to correctly verify user credentials.

SimpleCookie isn't recommended either because it makes it very easy for an attacker to spoof cookies.

A full example can be found in [example/][example/] directory

## Todo

The following items are in development or are planned to be developped:

* [ ] stateless cookie validation using JSON Web Tokens
* [ ] standard implementation for user storage into sqlite databases
* [x] publishing to crates.io
* [ ] cookie storage in Redis datastore
* [ ] API stability
* [ ] Documentation

