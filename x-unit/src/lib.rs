pub struct TestCase<F: Fn() -> ()> {
    test_method: F,
}

impl <F: Fn() -> ()> TestCase<F> {
    pub fn new(test_method: F) -> Self {
        TestCase { test_method }
    }

    pub fn run(&self) {
        (self.test_method)();
    }
}

pub struct WasRun<F: Fn() -> ()> {
    test_case: TestCase<F>,
    pub was_run: bool,
    pub was_setup: bool,
}

impl<F: Fn() -> ()> WasRun<F> {
    pub fn new(test_method: F) -> Self {
        WasRun {
            test_case: TestCase::new(test_method),
            was_run: false,
            was_setup: false,
        }
    }

    pub fn run(&mut self) {
        self.setup();
        self.test_case.run();
        self.was_run = true;
    }

    fn setup(&mut self) {
        self.was_setup = true;
        self.was_run = false;
    }
}