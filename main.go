package main

/*
#cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
#include "./C_headers/stack_vm.h"
*/
import "C"

import (
	"fmt"
	"github.com/andantan/hybrid-vm/ffi"
	"github.com/andantan/hybrid-vm/vm"
)

func main() {
	instructions := vm.NewInstructions()

	instructions.Push(vm.OpPush(-100))
	instructions.Push(vm.OpPush(55))
	instructions.Push(vm.OpAdd())
	instructions.Push(vm.OpHalt())

	stackVm, err := vm.NewVM(instructions)

	if err != nil {
		panic(err)
	}

	opResult := ffi.RunVM(stackVm.Ptr)

	if opResult.IsError {
		fmt.Println("Error occurred during VM execution")
	} else {
		fmt.Printf("VM execution successful. Result: %d\n", opResult.Result)
	}

	ffi.FreeVm(stackVm.Ptr)
}
