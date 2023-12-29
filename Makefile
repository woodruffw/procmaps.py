.PHONY: all
all:
	@echo "Run my targets individually!"

.PHONY: env
env:
	test -d env || python3 -m venv env
	. env/bin/activate && \
		python -m pip install maturin


.PHONY: develop
develop: env
	. env/bin/activate && \
		maturin develop

.PHONY: test
test: develop
	. env/bin/activate && \
		python -m unittest test/test_procmaps.py

.PHONY: build
build: env
	. env/bin/activate && \
		maturin build

.PHONY: dist
dist: env
	. env/bin/activate && \
		docker run --rm -v $(shell pwd):/io ghcr.io/pyo3/maturin build --release --sdist --strip --out dist
