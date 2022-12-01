package main

import (
  "fmt"
  "cuelang.org/go/cue"
  "cuelang.org/go/cue/cuecontext"
  "syscall/js"
)

func jsonWrapper() js.Func {
  jsonFunc := js.FuncOf(func(this js.Value, args []js.Value) any {
		fmt.Printf("unable to convert to json %s\n", err)
  })
  return jsonFunc
}

func main() {
  js.Global().Set("formatJSON", jsonWrapper())
  <-make(chan bool)
}
