extern crate xunit;

use xunit::*;

fn main() {
    TestCase::new(test_case_test).run();
}

fn was_run(test_data: &mut TestData) {
    test_data.insert("was_run".into(), true.into());
}

fn test_case_test(_: &mut TestData) {
    let mut test = TestCase::new(was_run);
    assert_eq!(test.get("was_run"), None);
    test.run();
    assert_eq!(test.get("was_run"), Some(&true.into()));
}