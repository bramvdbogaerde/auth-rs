use std;
use super::userpass::FromString;

pub trait Authenticator{
    // the type that is returned when someone calls user() on the authenticator
    // this can for example be a structure that represent the user that is logged in.
    // 
    // LoginStatus requires an implementator of this type to also implement ToString
    // this because the type must be serializable into a string in order to store it inside a
    // cookie.
    //
    // UserPass requires an implementator of this type to also implement FromString in order to
    // retreive the type back from the cookie string
    type User : FromString + ToString;

    /// a function that returns a user_id in the form a String
    fn user(&self) -> Self::User;

    /// a function that checks if the user pass combination is valid and if it is returns true and
    /// an instance of itself
    fn check_credentials(username: String, password: String) -> Result<Self,Self>
        where Self: std::marker::Sized;
}
