SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
NPM := npm

default: release

# Dependencies

yarn.lock: package.json
	$(NPM)

node_modules/.yarn-integrity: yarn.lock
	$(NPM) && $(NPM) upgrade components

# Development Workflow

.PHONY: dev
dev: node_modules/.yarn-integrity
	$(NPM) run dev

# Code Style

.PHONY: format
format: node_modules/.yarn-integrity
	$(NPM) run format


# Testing and Verification

.PHONY: check
check: node_modules/.yarn-integrity
	$(NPM) run check && $(NPM) run lint && $(NPM) run test

# Build and Release

.svelte-kit/output/client/vite-manifest.json: node_modules/.yarn-integrity postcss.config.cjs svelte.config.js tailwind.config.cjs vite.config.js $(wildcard src/**/*)
	$(NPM) run build

build: .svelte-kit/output/client/vite-manifest.json

release: check build

preview: release
	$(NPM) run preview

.PHONY: pretty
pretty:
	$(NPM) run format

.PHONY: clean
clean:
	rm -rf .svelte-kit/
