use super::config;

use rocket;
use rocket::request::FromRequest;
use rocket::Request;
use rocket::outcome::Outcome;

pub trait FromString{
    fn from_string(s: String) -> Self;
}

impl FromString for String{
    fn from_string(s: String) -> String{
        s
    }
}

pub struct UserPass<T>{
    pub user: T,
}

/// A request guard that checks if a private cookie was provided   
///
/// The name of the cookie can be configured with simpleauth_cookie_identifier config key in your
/// Rocket config file.
///
/// By default it is "sid" see the config module
impl<'a,'r, T: FromString> FromRequest<'a, 'r> for UserPass<T>{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<UserPass<T>,Self::Error>{
        let cookie_id = config::get_cookie_identifier();
        let mut cookies = request.cookies();

        match cookies.get_private(&cookie_id){
            Some(cookie) => Outcome::Success(UserPass{user: T::from_string(cookie.value().to_string())}),
            None => Outcome::Forward(())
        }
    }
}
