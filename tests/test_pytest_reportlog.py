import pytest
from test_results_parser import parse_pytest_reportlog, Testrun, Outcome


def test_reportlog():
    expected = [
        Testrun(
            "TestParsers.test_junit[./tests/junit.xml-expected0]",
            0.0009641647338867188,
            Outcome.Pass,
            "tests/test_parsers.py",
        ),
        Testrun(
            "TestParsers.test_junit[./tests/jest-junit.xml-expected1]",
            0.0008068084716796875,
            Outcome.Pass,
            "tests/test_parsers.py",
        ),
    ]

    with open("tests/log.jsonl", "b+r") as f:
        testruns = parse_pytest_reportlog(f.read())
        assert len(testruns) == len(expected)
        for restest, extest in zip(testruns, expected):
            assert restest == extest
