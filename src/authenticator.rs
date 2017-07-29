use std;

pub trait Authenticator{
    /// a function that returns a user_id in the form a String
    fn user_id(&self) -> String;

    /// a function that checks if the user pass combination is valid and if it is returns true and
    /// an instance of itself
    fn check_credentials(username: String, password: String) -> Result<Self,Self>
        where Self: std::marker::Sized;
}
