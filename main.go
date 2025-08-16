package main

/*
#cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
#include "./C_headers/stack_vm.h"
*/
import "C"

import (
	"fmt"
	"github.com/andantan/hybrid-vm/vm"
)

func main() {
	instructions := vm.NewInstructions()

	instructions.Push(vm.OpPushInt(-100))
	instructions.Push(vm.OpPushFloat(55.2))
	instructions.Push(vm.OpAdd())

	instructions.Push(vm.OpPushInt(100))
	instructions.Push(vm.OpPushFloat(-22.889))
	instructions.Push(vm.OpSub())

	instructions.Push(vm.OpAdd())

	instructions.Push(vm.OpPushFloat(10.3))
	instructions.Push(vm.OpMul())

	instructions.Push(vm.OpPushInt(-300000000))
	instructions.Push(vm.OpDiv())

	instructions.Push(vm.OpPushFloat(70.8))
	instructions.Push(vm.OpPushFloat(-88.188932))
	instructions.Push(vm.OpPushFloat(205.185))
	instructions.Push(vm.OpPushFloat(142.354))

	instructions.Push(vm.OpAdd())
	instructions.Push(vm.OpMul())
	instructions.Push(vm.OpDiv())
	instructions.Push(vm.OpSub())

	instructions.Push(vm.OpHalt())

	hvm, err := vm.NewVM(1<<10, instructions)

	if err != nil {
		panic(err)
	}

	result, err := hvm.Run()

	if err != nil {
		fmt.Printf("vm error: %s\n", err)
	} else {
		fmt.Printf("VM execution successful. Result: %+v\n", result)
	}
}
