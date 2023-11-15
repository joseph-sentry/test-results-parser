import pytest
from testing_result_parsers import parse_junit_xml, Testrun, Outcome


class TestParsers:
    @pytest.mark.parametrize(
        "filename,expected",
        [
            (
                "./tests/junit.xml",
                [
                    Testrun(
                        "tests.test_parsers.TestParsers::test_junit[junit.xml--True]",
                        0.001,
                        Outcome.Failure,
                        "pytest",
                    ),
                    Testrun(
                        "tests.test_parsers.TestParsers::test_junit[jest-junit.xml--False]",
                        0.064,
                        Outcome.Pass,
                        "pytest",
                    ),
                ],
            ),
            (
                "./tests/jest-junit.xml",
                [
                    Testrun(
                        "Title when rendered renders pull title::Title when rendered renders pull title",
                        0.036,
                        Outcome.Pass,
                        "Title",
                    ),
                    Testrun(
                        "Title when rendered renders pull author::Title when rendered renders pull author",
                        0.005,
                        Outcome.Pass,
                        "Title",
                    ),
                    Testrun(
                        "Title when rendered renders pull updatestamp::Title when rendered renders pull updatestamp",
                        0.002,
                        Outcome.Pass,
                        "Title",
                    ),
                    Testrun(
                        "Title when rendered for first pull request renders pull title::Title when rendered for first pull request renders pull title",
                        0.006,
                        Outcome.Pass,
                        "Title",
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
