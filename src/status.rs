use rocket::Response;
use rocket::response::Redirect;
use rocket::response::Responder;
use rocket::request::{FormItems, FromForm};
use rocket::http::Status;
use std::marker::PhantomData;
use std::collections::HashMap;
use super::cookie::{FromCookie,ToCookie};
use super::authenticator::Authenticator;

/// Login state is used after the user has typed its username and password. It checks with an
/// authenticator if given credentials are valid and returns InvalidCredentials and Succeed based
/// on the validality of the username and password.
pub enum LoginStatus<A,C>{
    Succeed(A,PhantomData<C>),
    Failed(A)
}

pub struct LoginRedirect<'r>(Response<'r>);

impl<A: Authenticator,C: ToCookie> LoginStatus<A,C>{
    /// Returns the user id from an instance of Authenticator
    pub fn get_authenticator (&self) -> &A{
        match self{
            &LoginStatus::Succeed(ref authenticator, _) => authenticator,
            &LoginStatus::Failed(ref authenticator) => authenticator
        }
    }

    /// Generates a succeed response 
    fn succeed<'r>(self, url: &'static str) -> Response<'r>{
        let redirect_response = Redirect::to(url);
        let mut builder = Response::build();

        builder
            .raw_header("Set-Cookie", format!("{}={}", A::COOKIE_IDENTIFIER, C::encode(A::SECRET.to_string(), self.get_authenticator().user_id())))
            .merge(redirect_response.respond().unwrap())
            .finalize()
    }

    /// Generates a failed response
    fn failed<'r>(self, url: &'static str) -> Response<'r>{
        let redirect_response = Redirect::to(url);
        redirect_response.respond().unwrap()
    }

    pub fn redirect<'r>(self,success_url: &'static str, failure_url: &'static str) -> LoginRedirect<'r>{
        let response = match self{
          LoginStatus::Succeed(_,_) => self.succeed::<'r>(success_url),
          LoginStatus::Failed(_) => self.failed::<'r>(failure_url)
        };

        LoginRedirect(response)        
    }
}       

impl<'f,A: Authenticator,C> FromForm<'f> for LoginStatus<A,C>{
    type Error = &'static str;
    
    fn from_form_string(form_string: &'f str) -> Result<Self, Self::Error>{
        let mut form_items = FormItems::<'f>(form_string);
        let mut user_pass = HashMap::new();

        for (key,value) in form_items{
            println!("{} - {}", key,value);
            match key{
                "username" => user_pass.insert("username", value).map_or((), |v| ()),
                "password" => user_pass.insert("password", value).map_or((), |v| ()),
                _ => ()
            }
        }

        if user_pass.get("username").is_none() || user_pass.get("password").is_none() {
            Err("invalid form")
        } else {
            let result = A::check_credentials(user_pass.get("username").unwrap().to_string(), user_pass.get("password").unwrap().to_string());

            Ok(match result{
                Ok(authenticator) => LoginStatus::Succeed(authenticator, PhantomData),
                Err(authenticator) => LoginStatus::Failed(authenticator)
            })
        }
    }
}

impl<'r> Responder<'r> for LoginRedirect<'r>{
    fn respond(self) -> Result<Response<'r>, Status>{
        Ok(self.0)
    }
}
