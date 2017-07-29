use super::authenticator::Authenticator;

pub struct DummyAuthenticator{}

impl Authenticator for DummyAuthenticator{
    fn user_id(&self) -> String{
        "hello world".to_string()
    }   

    fn check_credentials(_username: String, _password: String) -> Result<Self,Self>{
        Ok(DummyAuthenticator{})
    }
}
