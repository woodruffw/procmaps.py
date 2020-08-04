.PHONY: all
all:
	@echo "Run my targets individually!"

.PHONY: env
.ONESHELL:
env:
	test -d env || python3 -m venv env
	. env/bin/activate
	pip install maturin


.PHONY: develop
.ONESHELL:
develop: env
	. env/bin/activate
	maturin develop

.PHONY: test
.ONESHELL:
test: develop
	. env/bin/activate
	python -m unittest test/test_procmaps.py

.PHONY: build
.ONESHELL:
build: env
	. env/bin/activate
	maturin build

.PHONY: dist
.ONESHELL:
dist: env
	. env/bin/activate
	docker run --rm -v $(shell pwd):/io konstin2/maturin build --release --strip
