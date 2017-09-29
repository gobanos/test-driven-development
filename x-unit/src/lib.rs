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

pub struct TestCase<F: Fn(&mut TestData) -> ()> {
    test_data: TestData,
    test_method: F,
}

impl<F: Fn(&mut TestData) -> ()> TestCase<F> {
    pub fn new(test_method: F) -> Self {
        TestCase { test_data: TestData::new(), test_method }
    }

    pub fn run(&mut self) {
        (self.test_method)(&mut self.test_data);
    }

    pub fn get(&self, key: &str) -> Option<&TestValue> {
        self.test_data.get(key)
    }
}