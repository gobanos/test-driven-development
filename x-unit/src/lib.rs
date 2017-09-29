use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TestValue {
    Bool(bool),
}

impl From<bool> for TestValue {
    fn from(b: bool) -> Self {
        TestValue::Bool(b)
    }
}

pub type TestData = HashMap<String, TestValue>;

pub struct TestCase<S, F> where
    S: Fn(&mut TestData) -> (),
    F: Fn(&TestData, &mut TestData) -> (),
{
    pub setup: S,
    pub run: F,
}

pub struct TestRunner<S, F> where
    S: Fn(&mut TestData) -> (),
    F: Fn(&TestData, &mut TestData) -> (),
{
    test_setup: TestData,
    test_data: TestData,
    test: TestCase<S, F>,
}

impl<S, F> TestRunner<S, F> where
    S: Fn(&mut TestData) -> (),
    F: Fn(&TestData, &mut TestData) -> (),
{
    pub fn new(test: TestCase<S, F>) -> Self {
        TestRunner {
            test_setup: TestData::new(),
            test_data: TestData::new(),
            test,
        }
    }

    pub fn setup(&mut self) {
        (self.test.setup)(&mut self.test_setup);
    }

    pub fn run(&mut self) {
        (self.test.run)(&self.test_setup, &mut self.test_data);
    }

    pub fn get(&self, key: &str) -> Option<&TestValue> {
        self.test_data.get(key)
    }
}