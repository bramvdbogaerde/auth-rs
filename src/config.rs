use rocket::config::{Config, Environment};

const COOKIE_IDENTIFIER_CONFIG_KEY : &'static str = "simpleauth_cookie_identifier";

fn get_config() -> Config{
    Config::build(Environment::active().unwrap())
        .extra(COOKIE_IDENTIFIER_CONFIG_KEY, "sid")
        .unwrap()
}


pub fn get_cookie_identifier() -> String{
    let config = get_config();
    config.get_str(COOKIE_IDENTIFIER_CONFIG_KEY).unwrap().to_owned()
}
