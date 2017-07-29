extern crate rocket;

pub mod authenticator;

pub mod status;
pub mod userpass;

/// Example implementation of Authenticator and FromCookie and ToCookie
pub mod dummy;
mod config;

