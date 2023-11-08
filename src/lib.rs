use std::fmt::Debug;
use std::sync::mpsc::channel;
pub enum Error<'a> {
    Fail {
        msg: String,
        test: Option<Box<dyn Test + 'a>>,
    },
}

pub trait Test: Debug + Sync + Send {
    fn test<'t>(self: Box<Self>) -> Result<String, Error<'t>>;
}

pub struct Suite<'s> {
    tests: Vec<Box<dyn Test + 's>>,
}

impl<'s> Suite<'s> {
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }

    pub fn add(&mut self, test: impl Test + 's) {
        self.tests.push(Box::new(test));
    }
}

impl<'s> Default for Suite<'s> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s, T> Extend<T> for Suite<'s>
where
    T: Test + 's,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for test in iter {
            self.add(test);
        }
    }
}

impl<'s> Extend<Box<dyn Test + 's>> for Suite<'s> {
    fn extend<T: IntoIterator<Item = Box<dyn Test + 's>>>(&mut self, iter: T) {
        self.tests.extend(iter)
    }
}

impl<'s, T> FromIterator<T> for Suite<'s>
where
    T: Test + 's,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut suite = Suite::new();
        suite.extend(iter);
        suite
    }
}

impl<'s> FromIterator<Box<dyn Test + 's>> for Suite<'s> {
    fn from_iter<T: IntoIterator<Item = Box<dyn Test + 's>>>(iter: T) -> Self {
        let mut suite = Suite::new();
        suite.extend(iter);
        suite
    }
}

impl<'s> FromIterator<Error<'s>> for Suite<'s> {
    fn from_iter<T: IntoIterator<Item = Error<'s>>>(iter: T) -> Self {
        Suite::from_iter(iter.into_iter().flat_map(|e| {
            let Error::Fail { test, .. } = e;
            test
        }))
    }
}

impl<'s> Suite<'s> {
    pub fn run(&mut self) -> Option<Vec<Error<'s>>> {
        let (tx, rx) = channel();
        rayon::scope(|s| {
            while let Some(test) = self.tests.pop() {
                s.spawn(|_| {
                    tx.send(test.test()).unwrap();
                });
            }
        });

        // Drop the last instance of tx end of the channel.
        // Otherwise the iterator below will hang indefinitely.
        drop(tx);

        let mut failed = Vec::new();
        for res in rx {
            match res {
                Ok(msg) => eprintln!("{msg}"),
                Err(e) => {
                    let Error::Fail { msg, .. } = &e;
                    eprintln!("FAIL {}", msg);
                    failed.push(e);
                }
            }
        }
        (!failed.is_empty()).then_some(failed)
    }
}
