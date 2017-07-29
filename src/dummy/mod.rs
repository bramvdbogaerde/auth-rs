use super::authenticator::Authenticator;

pub struct DummyAuthenticator{}

impl Authenticator for DummyAuthenticator{
    type User = String;

    fn user(&self) -> String{
        "hello world".to_string()
    }   

    fn check_credentials(_username: String, _password: String) -> Result<Self,Self>{
        Ok(DummyAuthenticator{})
    }
}
