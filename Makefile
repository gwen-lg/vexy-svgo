# this_file: Makefile

.PHONY: docs-serve docs-build docs-clean

docs-serve:
	mkdocs serve -a localhost:8000

docs-build:
	mkdocs build

docs-clean:
	rm -rf docs/*

docs-rebuild: docs-clean docs-build