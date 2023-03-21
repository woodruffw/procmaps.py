procmaps.py
===========

[![CI](https://github.com/woodruffw/procmaps.py/actions/workflows/ci.yml/badge.svg)](https://github.com/woodruffw/procmaps.py/actions/workflows/ci.yml)
[![Downloads](https://pepy.tech/badge/procmaps)](https://pepy.tech/project/procmaps)

Python bindings for [procmaps.rs](https://github.com/woodruffw/procmaps.rs).

## Installation

procmaps.py is available for Python 3.7+ via pip:

```bash
$ pip install procmaps
```

## Usage

```python
import procmaps

# also: from_path, from_str
maps = procmaps.from_pid(9001)
for map_ in maps:
    # `in` can be used to check address inclusion
    if 0xfeedface in map_:
        print("this map contains some address!")

    # see dict(map_) for all attributes
    print(f"{map_.begin_address}: {map_.pathname}")
```

## Development

*procmaps.py* uses [PyO3](https://github.com/PyO3/pyo3) for binding Rust
to Python. It uses [Maturin](https://github.com/PyO3/maturin) to coerce the
Rust build into a `pip` and PyPI-compatible wheel.

Assuming that you have Rust and a relatively recent Python 3 installed,
the following should just work:

```bash
$ make develop
$ source env/bin/activate
```

A local build of *procmaps.py* will be created and installed in your virtual environment.
