use super::authenticator::Authenticator;
use std::marker::PhantomData;

use rocket;
use rocket::request::FromRequest;
use rocket::Request;
use rocket::outcome::Outcome;

pub struct UserPass<A>{
    user_id: String,
    authenticator: PhantomData<A>
}

/// A request guard that checks if a private cookie was provided
impl<'a,'r,A: Authenticator> FromRequest<'a, 'r> for UserPass<A>{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self,Self::Error>{
        let mut cookies = request.cookies();
        match cookies.get_private(A::COOKIE_IDENTIFIER){
            Some(cookie) => Outcome::Success(UserPass{user_id: cookie.value().to_string(), authenticator: PhantomData}),
            None => Outcome::Forward(())
        }
    }
}
