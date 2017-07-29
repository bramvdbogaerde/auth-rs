use super::config;
use rocket::Response;
use rocket::response::Redirect;
use rocket::response::Responder;
use rocket::request::{FormItems, FromForm, Request};
use rocket::http::{Status, Cookie, Cookies};
use std::collections::HashMap;
use super::authenticator::Authenticator;


/// Login state is used after the user has typed its username and password. It checks with an
/// authenticator if given credentials are valid and returns InvalidCredentials or Succeed based
/// on the validality of the username and password.
///
/// It does that by implementing the FromForm trait that takes the form submitted by your login
/// page
///
/// It expects a form like this on your page:
///
///```
///<form>
/// <input type="text" name="username" />
/// <input type="password" name="password" />
///</form>
/// ```
pub enum LoginStatus<A>{
    Succeed(A),
    Failed(A)
}

pub struct LoginRedirect(Redirect);

impl<A: Authenticator> LoginStatus<A> {
    /// Returns the user id from an instance of Authenticator
    pub fn get_authenticator (&self) -> &A{
        match self{
            &LoginStatus::Succeed(ref authenticator) => authenticator,
            &LoginStatus::Failed(ref authenticator) => authenticator
        }
    }

    /// Generates a succeed response
    fn succeed(self, url: &'static str, mut cookies: Cookies) -> Redirect {
        let cookie_identifier = config::get_cookie_identifier();

        cookies.add_private(Cookie::new(cookie_identifier, self.get_authenticator().user().to_string()));
        Redirect::to(url)
    }

    /// Generates a failed response
    fn failed(self, url: &'static str) -> Redirect {
        Redirect::to(url)
    }

    pub fn redirect(self, success_url: &'static str, failure_url: &'static str, cookies: Cookies) -> LoginRedirect{
        let redirect = match self {
          LoginStatus::Succeed(_) => self.succeed(success_url, cookies),
          LoginStatus::Failed(_) => self.failed(failure_url)
        };

        LoginRedirect(redirect)
    }
}

impl<'f,A: Authenticator> FromForm<'f> for LoginStatus<A>{
    type Error = &'static str;
    
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        let mut user_pass = HashMap::new();

        for (key,value) in form_items{
            match key.as_str(){
                "username" => user_pass.insert("username", value).map_or((), |_v| ()),
                "password" => user_pass.insert("password", value).map_or((), |_v| ()),
                _ => ()
            }
        }

        if user_pass.get("username").is_none() || user_pass.get("password").is_none() {
            Err("invalid form")
        } else {
            let result = A::check_credentials(user_pass.get("username").unwrap().to_string(), user_pass.get("password").unwrap().to_string());

            Ok(match result{
                Ok(authenticator) => LoginStatus::Succeed(authenticator),
                Err(authenticator) => LoginStatus::Failed(authenticator)
            })
        }
    }
}

impl<'r> Responder<'r> for LoginRedirect{
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status>{
        self.0.respond_to(request)
    }
}
