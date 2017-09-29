extern crate xunit;

use xunit::*;

fn main() {
    TestRunner::new(TestCase { setup: |_| {}, run: test_case_test }).run();
}

fn was_run(_: &TestData, test_data: &mut TestData) {
    test_data.insert("was_run".into(), true.into());
}

fn test_case_test(_: &TestData, _: &mut TestData) {
    let mut test = TestRunner::new(TestCase { setup: |_| {}, run: was_run });
    assert_eq!(test.get("was_run"), None);
    test.run();
    assert_eq!(test.get("was_run"), Some(&true.into()));
}