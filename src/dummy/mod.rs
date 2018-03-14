use super::authenticator::Authenticator;

pub struct DummyAuthenticator{}

/// An implementation of the authenticator
/// which always lets the authentication succeed
///
/// On every invocation this will also print the incoming
/// username and password.
///
/// This type should only be used for testing purposes.
impl Authenticator for DummyAuthenticator{
    type User = String;

    fn user(&self) -> String{
        "hello world".to_string()
    }   

    fn check_credentials(_username: String, _password: String) -> Result<Self,Self>{
        println!("username: {}, password: {}", _username, _password);

        Ok(DummyAuthenticator{})
    }
}
