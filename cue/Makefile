SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

GRADLE ?= ../gradlew

default: build

go.sum: go.mod $(CUE_WASM_INPUT) $(CUE_C_INPUT)
	go mod tidy

CUE_WASM_INPUT := wasm/cue_wasm.go
CUE_WASM_OUTPUT := cue.wasm

$(CUE_WASM_OUTPUT): $(CUE_WASM_INPUT)
	GOOS=js GOARCH=wasm go build -o $(CUE_WASM_OUTPUT) $(CUE_WASM_INPUT)

CUE_C_INPUT := c/cue_c.go
CUE_C_OUTPUT := libcue.a

$(CUE_C_OUTPUT): $(CUE_C_INPUT)
	go build -buildmode=c-archive -o $(CUE_C_OUTPUT) $(CUE_C_INPUT)

build: $(CUE_WASM_OUTPUT) $(CUE_C_OUTPUT)
