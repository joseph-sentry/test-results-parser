This project requires `maturin`, it can be installed using pip:
```pip install maturin```

For development, to install the library in your local venv (I haven't gotten this to work without using a venv), run:
```maturin develop```

this will rebuild the rust portion of the library

To use the library just import it after installing:
```import testing_result_parsers```

The CI uses the maturin-action to build wheels and an sdist

The version of the wheels built are determined by the value of the version in the cargo.toml