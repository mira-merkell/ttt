use ttt::test::{self, Test};
use ttt::Suite;

fn add(x: usize, y: usize) -> usize {
    x + y
}

#[derive(Debug)]
struct TestAdd(usize, usize);

impl Test for TestAdd {
    fn test<'t>(self: Box<Self>) -> Result<(), test::Error<'t>> {
        if add(self.0, self.1) != self.0 + self.1 {
            Err(test::Error::Fail {
                test: Some(self),
                msg: "addition",
            })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
struct TestFail;

impl Test for TestFail {
    fn test<'t>(self: Box<Self>) -> Result<(), test::Error<'t>> {
        Err(test::Error::Fail {
            test: None,
            msg: "fail",
        })
    }
}

#[test]
fn test_suite() {
    let mut suite: Suite<'_> = (0..3).map(|i| TestAdd(i, i + 1)).collect();
    suite.add(TestFail);
    for i in 0..3 {
        suite.add(TestAdd(2 * i, 2 * i))
    }

    let failed = suite.run().unwrap();
    assert_eq!(failed.len(), 1);
}
