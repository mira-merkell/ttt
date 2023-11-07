use crate::test::Test;

pub mod test;

pub struct Suite<'s> {
    tests: Vec<Box<dyn Test + 's>>,
    failed: Option<Vec<test::Error<'s>>>,
}

impl<'s> Suite<'s> {
    pub fn run(&mut self) {
        while let Some(test) = self.tests.pop() {
            if let Err(e) = test.test() {
                if let Some(failed) = &mut self.failed {
                    failed.push(e)
                } else {
                    let _ = std::mem::replace(&mut self.failed, Some(vec![e]));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
