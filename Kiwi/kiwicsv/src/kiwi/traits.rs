use std::borrow::Cow;

pub trait Serialize<'a> {
    fn serialize(&self) -> Cow<'a, str>;
}

pub trait Deserialize<'a, T>
where
    T: std::fmt::Display,
{
    type Error;
    fn deserialize(content: &str) -> Result<T, Self::Error>;
}
