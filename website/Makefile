SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
YARN := yarn

default: release

# Dependencies

../components/package/package.json:
	cd ../components && $(MAKE) build

yarn.lock:
	$(YARN)

node_modules/.yarn-integrity: package.json yarn.lock ../components/package/package.json
	$(YARN) && $(YARN) upgrade components

# Development Workflow

.PHONY: dev
dev: node_modules/.yarn-integrity
	$(YARN) dev

# Code Style

.PHONY: format
format: node_modules/.yarn-integrity
	$(YARN) format


# Testing and Verification

.PHONY: check
check: node_modules/.yarn-integrity
	$(YARN) run check && $(YARN) run lint && $(YARN) run test

# Build and Release

.svelte-kit/output/client/vite-manifest.json: node_modules/.yarn-integrity postcss.config.cjs svelte.config.js tailwind.config.cjs vite.config.js $(wildcard src/**/*)
	$(YARN) run build

build: .svelte-kit/output/client/vite-manifest.json

release: check build

preview: release
	$(YARN) preview

.PHONY: clean
clean:
	rm -rf .svelte-kit/
