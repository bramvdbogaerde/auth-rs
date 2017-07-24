use super::authenticator::Authenticator;

pub struct DummyAuthenticator{}

impl Authenticator for DummyAuthenticator{
    const COOKIE_IDENTIFIER : &'static str = "dummy.sessionid";

    fn user_id(&self) -> String{
        "hello world".to_string()
    }   

    fn check_credentials(_username: String, _password: String) -> Result<Self,Self>{
        Ok(DummyAuthenticator{})
    }
}
