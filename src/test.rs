use std::fmt::Debug;

pub trait Test: Debug + Sync + Send {
    fn test<'t>(self: Box<Self>) -> Result<(), Error<'t>>;
}

pub enum Error<'a> {
    Fail {
        test: Option<Box<dyn Test + 'a>>,
        msg: &'a str,
    },
}
