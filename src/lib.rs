use std::sync::mpsc::channel;

pub use crate::test::Test;

pub mod test;

pub struct Suite<'s> {
    tests: Vec<Box<dyn Test + 's>>,
}

impl<'s> Default for Suite<'s> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> Suite<'s> {
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }

    pub fn add(&mut self, test: impl Test + 's) {
        self.tests.push(Box::new(test));
    }
}

impl<'s, T> FromIterator<T> for Suite<'s>
where
    T: Test + 's,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut suite = Suite::new();
        for test in iter {
            suite.add(test);
        }
        suite
    }
}

impl<'s> Suite<'s> {
    pub fn run(&mut self) -> Option<Vec<test::Error<'s>>> {
        let (tx, rx) = channel();
        rayon::scope(|s| {
            while let Some(test) = self.tests.pop() {
                s.spawn(|_| {
                    let test_name = format!("{:?}", test);
                    tx.send(test.test().map(|_| test_name)).unwrap();
                });
            }
        });
        let mut failed = Vec::new();

        // Drop the last instance of tx end of the channel.
        // Otherwise the iterator below will hang indefinitely.
        drop(tx);
        for res in rx {
            match res {
                Ok(msg) => eprintln!("OK {}", msg),
                Err(e) => {
                    eprintln!("FAIL");
                    failed.push(e);
                }
            }
        }
        (!failed.is_empty()).then_some(failed)
    }
}
