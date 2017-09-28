extern crate xunit;

use xunit::*;

fn main() {
    let test = TestCase::new(|| {
        let mut test = WasRun::new(dummy);
        assert!(!test.was_setup);
        assert!(!test.was_run);
        test.run();
        assert!(test.was_setup);
        assert!(test.was_run);
    });

    test.run();
}

fn dummy() {  }