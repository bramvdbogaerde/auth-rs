/// Types that implement this trait provide a way to validate and parse a cookie string
pub trait FromCookie{
    type Error;
    fn is_valid(secret: String, value: String) -> Result<String, Self::Error>;
}

/// Types that implement this trait provide a way to encode a value into a cookie string using a
/// secret
pub trait ToCookie{
    fn encode(secret: String, value: String) -> String;
}
