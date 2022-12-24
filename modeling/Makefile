SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

CARGO ?= cargo

default: build

# Deps

editor-app/yarn.lock: editor-app/package.json editor-state/pkg/package.json
	cd editor-app && yarn

# Dev

.PHONY: storybook
storybook: editor-app/yarn.lock
	cd editor-app && yarn storybook

.PHONY: dev
dev: editor-app/yarn.lock
	cd editor-app && yarn dev

# Verification

.PHONY: check
check: editor-app/yarn.lock
	$(CARGO) :editor-state:check
	cd editor-app && yarn run check && yarn run lint && yarn run test

# Build

editor-state/build/productionLibrary/package.json: editor-state/build.gradle.kts editor-state/gradle.properties $(wildcard editor-state/src/**/*)
	$(GRADLE) :editor-state:build
	cd editor-app && yarn upgrade editor-state

editor-app/.svelte-kit/cloudflare/vite-manifest.json: editor-app/yarn.lock editor-state/build/productionLibrary/package.json $(wildcard editor-app/src/**/*)
	cd editor-app && yarn run build

build: check editor-app/.svelte-kit/cloudflare/vite-manifest.json

clean:
	rm -rf editor-app/.svelte-kit
	$(GRADLE) :editor-state:clean
