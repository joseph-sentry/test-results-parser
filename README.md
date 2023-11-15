This project requires `maturin`, it can be installed using pip:
```pip install maturin```

For development, to rebuild the rust portion of the library and install the library in your local venv (I believe this _requires_ a venv in order to work), run:
```maturin develop```

To use the library just import it after installing:
```import testing_result_parsers```

There's an example of this in the tests directory.

The CI uses the maturin-action to build wheels and an sdist

The version of the wheels built are determined by the value of the version in the cargo.toml

There are 2 parsing function currently implemented:

- `parse_junit_xml`: this parses `junit.xml` files
- `parse_pytest_reportlog`: this parses files produced by the `pytest-reportlog` extension

Both these functions take the path to the file to parse as an arg and return a list of `Testrun` objects.

The `Testrun` objects look like this:
```
Outcome:
    Pass,
    Failure,
    Error,
    Skip

Testrun:
    name: str
    outcome: Outcome
    duration: float
    testsuite: str
```