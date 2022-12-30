SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

CARGO ?= cargo
WASM_PACK ?= wasm-pack
YARN ?= yarn

default: build

# Deps

editor-app/yarn.lock: editor-app/package.json editor-state/pkg/package.json
	cd editor-app && $(YARN)

editor-app/node_modules/.yarn-integrity: editor-app/yarn.lock
	cd editor-app && $(YARN) && $(YARN) upgrade components

# Dev

.PHONY: dev
dev: editor-app/node_modules/.yarn-integrity
	cd editor-app && $(YARN) dev

# Verification

.PHONY: check
check: editor-app/node_modules/.yarn-integrity
	cd editor-state && $(WASM_PACK) test --headless --firefox
	cd editor-app && $(YARN) run check && $(YARN) run lint && $(YARN) run test

# Build

editor-state/pkg/package.json: editor-state/Cargo.lock $(wildcard editor-state/src/**/*)
	cd editor-state && $(WASM_PACK) build

editor-app/.svelte-kit/cloudflare/vite-manifest.json: editor-app/node_modules/.yarn-integrity $(wildcard editor-app/src/**/*)
	cd editor-app && $(YARN) run build

build: check editor-app/.svelte-kit/cloudflare/vite-manifest.json

clean:
	rm -rf editor-app/.svelte-kit
	cd editor-state && $(CARGO) clean
