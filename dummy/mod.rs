use super::authenticator::Authenticator;
use super::cookie::{ToCookie,FromCookie};

pub struct DummyAuthenticator{}

pub struct SimpleCookie{}

impl Authenticator for DummyAuthenticator{
    const COOKIE_IDENTIFIER : &'static str = "dummy.sessionid";
    const SECRET : &'static str = "changme";

    fn user_id(&self) -> String{
        "hello world".to_string()
    }   

    fn check_credentials(username: String, password: String) -> Result<Self,Self>{
        Ok(DummyAuthenticator{})
    }
}

impl ToCookie for SimpleCookie{
    fn encode(secret: String, value: String) -> String{
        value
    }
}

impl FromCookie for SimpleCookie{
    type Error = ();
    fn is_valid (secret: String, value: String) -> Result<String, Self::Error>{
        Ok(value)
    }
}
