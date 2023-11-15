import pytest
from testing_result_parsers import parse_pytest_reportlog, Testrun


def test_reportlog():
    expected = [
        Testrun(
            "TestParsers.test_junit[./tests/junit.xml-expected0]",
            "0.0009641647338867188",
            "passed",
            "tests/test_parsers.py",
        ),
        Testrun(
            "TestParsers.test_junit[./tests/jest-junit.xml-expected1]",
            "0.0008068084716796875",
            "passed",
            "tests/test_parsers.py",
        ),
    ]

    testruns = parse_pytest_reportlog("tests/log.jsonl")
    assert len(testruns) == len(expected)
    for restest, extest in zip(testruns, expected):
        assert restest == extest
