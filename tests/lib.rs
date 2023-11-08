use ttt::{Suite, Test};

fn add(x: usize, y: usize) -> usize {
    x + y
}

#[derive(Debug)]
struct TestAdd(usize, usize);

impl Test for TestAdd {
    fn test<'t>(self: Box<Self>) -> Result<String, ttt::Error<'t>> {
        if add(self.0, self.1) != self.0 + self.1 {
            Err(ttt::Error::Fail {
                test: Some(self),
                msg: "addition".to_string(),
            })
        } else {
            Ok("OK".to_string())
        }
    }
}

#[derive(Debug)]
struct TestFail;

impl Test for TestFail {
    fn test<'t>(self: Box<Self>) -> Result<String, ttt::Error<'t>> {
        Err(ttt::Error::Fail {
            test: Some(self),
            msg: "fail".to_string(),
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

    let mut suite = Suite::from_iter(failed);
    assert!(suite.run().is_some());
}
