import pytest
from testing_result_parsers import parse_vitest_json, Testrun, Outcome


def test_vitest_json():
    expected = [
        Testrun(
            " first test file 2 + 2 should equal 4",
            0.009,
            Outcome.Failure,
            "/root-directory/__tests__/test-file-1.test.ts",
        ),
        Testrun(
            " first test file 2 + 2 should equal 4",
            0.009,
            Outcome.Failure,
            "/root-directory/__tests__/test-file-1.test.ts",
        ),
        Testrun(
            " first test file 2 + 2 should equal 4",
            0.009,
            Outcome.Failure,
            "/root-directory/__tests__/test-file-1.test.ts",
        ),
        Testrun(
            " first test file 2 + 2 should equal 4",
            0.009,
            Outcome.Failure,
            "/root-directory/__tests__/test-file-1.test.ts",
        ),
    ]

    testruns = parse_vitest_json("tests/vitest.json")

    assert len(testruns) == len(expected)
    for restest, extest in zip(testruns, expected):
        assert restest == extest
