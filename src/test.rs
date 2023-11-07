use std::fmt::Debug;

pub trait Test: Debug {
    fn test<'t>(self: Box<Self>) -> Result<(), Error<'t>>;
}

pub enum Error<'a> {
    Fail {
        test: Box<dyn Test + 'a>,
        msg: &'a str,
    },
}
