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

Cargo.toml
```
[dependencies]
rocket = "0.2.6"
rocket-simpleauth = {path = "../"}
rocket_codegen = "0.2.6"
```

main.rs
```
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_simpleauth as auth;
extern crate rocket;

use auth::userpass::UserPass;
use rocket::request::Form;
use auth::status::{LoginStatus,LoginRedirect};
use auth::dummy::{DummyAuthenticator,SimpleCookie};

#[get("/admin")]
fn admin(user: UserPass<DummyAuthenticator, SimpleCookie>) -> &'static str{
	// we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
	"Restricted administration area"
}

#[get("/admin")]
fn login() -> &'static str{
	"Login page"
}

#[post("/admin", data = "<form>")]
fn login_post(form: Form<LoginStatus<DummyAuthenticator, SimpleCookie>>) -> LoginRedirect{
	// creates a response with either a cookie set (in case of a succesfull login)
	// or not (in case of a failure). In both cases a "Location" header is send.
	// the first parameter indicates the redirect URL when successful login,
	// the second a URL for a failed login
	form.into_inner().redirect("/admin", "/admin")
}

fn main(){
    // main setup code
}
```

A full example can be found in [example/](example/) directory

## Todo

The following items are in development or are planned to be developped:

* [ ] stateless cookie validation using JSON Web Tokens
* [ ] standard implementation for user storage into sqlite databases
* [x] publishing to crates.io
* [ ] cookie storage in Redis datastore
* [ ] API stability
* [ ] Documentation

