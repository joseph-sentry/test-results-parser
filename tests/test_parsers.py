import pytest
from testing_result_parsers import parse_junit_xml, Testrun


class TestParsers:
    @pytest.mark.parametrize(
        "filename,expected",
        [
            (
                "./tests/junit.xml",
                [
                    Testrun(
                        name="tests.test_parsers.TestParsers::test_junit[junit.xml--True]",
                        duration="0.001",
                        outcome="failure",
                        testsuite="pytest",
                    ),
                    Testrun(
                        name="tests.test_parsers.TestParsers::test_junit[jest-junit.xml--False]",
                        duration="0.064",
                        outcome="pass",
                        testsuite="pytest",
                    ),
                ],
            ),
            (
                "./tests/jest-junit.xml",
                [
                    Testrun(
                        name="Title when rendered renders pull title::Title when rendered renders pull title",
                        duration="0.036",
                        outcome="pass",
                        testsuite="Title",
                    ),
                    Testrun(
                        name="Title when rendered renders pull author::Title when rendered renders pull author",
                        duration="0.005",
                        outcome="pass",
                        testsuite="Title",
                    ),
                    Testrun(
                        name="Title when rendered renders pull updatestamp::Title when rendered renders pull updatestamp",
                        duration="0.002",
                        outcome="pass",
                        testsuite="Title",
                    ),
                    Testrun(
                        name="Title when rendered for first pull request renders pull title::Title when rendered for first pull request renders pull title",
                        duration="0.006",
                        outcome="pass",
                        testsuite="Title",
                    ),
                ],
            ),
        ],
    )
    def test_junit(self, filename, expected):
        res = parse_junit_xml(filename)
        assert len(res) == len(expected)
        for restest, extest in zip(res, expected):
            assert restest == extest
