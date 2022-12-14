SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
YARN := yarn

default: build

yarn.lock:
	$(YARN)

node_modules/.yarn-integrity: package.json yarn.lock
	$(YARN)

.PHONY: check
check: node_modules/.yarn-integrity
	$(YARN) run check && $(YARN) run lint && $(YARN) run test

.PHONY: storybook
storybook: node_modules/.yarn-integrity
	$(YARN) storybook

.PHONY: dev
dev: node_modules/.yarn-integrity
	$(YARN) dev

package/package.json: node_modules/.yarn-integrity $(wildcard app/src/**/*)
	$(YARN) run build

build: check package/package.json

clean:
	rm -rf package/
