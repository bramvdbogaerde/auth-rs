#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_simpleauth as auth;
extern crate rocket;

use auth::userpass::UserPass;
use rocket::request::Form;
use auth::status::{LoginStatus,LoginRedirect};
use auth::dummy::{DummyAuthenticator,SimpleCookie};

#[get("/admin")]
fn admin(user: UserPass<DummyAuthenticator, SimpleCookie>) -> &'static str{
	// we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
	"Restricted administration area"
}

#[get("/admin")]
fn login() -> &'static str{
	"Login page"
}

#[post("/admin", data = "<form>")]
fn login_post(form: Form<LoginStatus<DummyAuthenticator, SimpleCookie>>) -> LoginRedirect{
	// creates a response with either a cookie set (in case of a succesfull login)
	// or not (in case of a failure). In both cases a "Location" header is send.
	// the first parameter indicates the redirect URL when successful login,
	// the second a URL for a failed login
	form.into_inner().redirect("/admin", "/admin")
}

fn main(){
    // main setup code
}
