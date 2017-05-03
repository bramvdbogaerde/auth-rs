use super::authenticator::Authenticator;
use super::cookie::{FromCookie,ToCookie};
use std::marker::PhantomData;

use rocket;
use rocket::request::FromRequest;
use rocket::Request;
use rocket::outcome::Outcome;

pub struct UserPass<A,C>{
    user_id: String,
    authenticator: PhantomData<A>,
    cookie: PhantomData<C>
}

/// A request guard that checks if a cookie with a valid jwt token was provided
impl<'a,'r,A: Authenticator, C: FromCookie> FromRequest<'a, 'r> for UserPass<A,C>{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self,Self::Error>{
        let cookies = request.cookies();
        match cookies.find(A::COOKIE_IDENTIFIER){
            Some(cookie) => match C::is_valid(A::SECRET.to_string(), cookie.value().to_string()){
                Ok(info) => Outcome::Success(UserPass{user_id: info, authenticator: PhantomData, cookie: PhantomData}),
                Err(_) => Outcome::Forward(())
            },
            None => Outcome::Forward(())
        }
    }
}
