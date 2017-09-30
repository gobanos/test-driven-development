class TestCase:
    def __init__(self, name):
        self.name = name

    def setUp(self):
        pass

    def run(self):
        result = TestResult()
        result.testStarted()
        self.setUp()
        method = getattr(self, self.name)
        method()
        self.tearDown()
        return result

    def tearDown(self):
        pass


class WasRun(TestCase):
    def __init__(self, name):
        TestCase.__init__(self, name)

    def setUp(self):
        self.wasRun = None
        self.log = "setUp "

    def tearDown(self):
        self.log = self.log + "tearDown "

    def testMethod(self):
        self.wasRun = 1
        self.log = self.log + "testMethod "

    def testBrokenMethod(self):
        raise Exception


class TestResult:
    def __init__(self):
        self.runCount = 0

    def testStarted(self):
        self.runCount = self.runCount + 1

    def summary(self):
        return "%d run, 0 failed" % self.runCount


class TestCaseTest(TestCase):
    def testTemplateMethod(self):
        test = WasRun("testMethod")
        test.run()
        assert(test.log == "setUp testMethod tearDown ")

    def testResult(self):
        test = WasRun("testMethod")
        result = test.run()
        assert(result.summary() == "1 run, 0 failed")

    def testFailedResult(self):
        test = WasRun("testBrokenMethod")
        result = test.run()
        assert(result.summary() == "1 run, 1 failed")


TestCaseTest("testTemplateMethod").run()
TestCaseTest("testResult").run()
TestCaseTest("testFailedResult").run()
