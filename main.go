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

	b1 := []byte{
		0x00, 0x01, 0x02, 0x03,
		0x04, 0x05, 0x06, 0x07,
		0x08, 0x09, 0x0A, 0x0B,
		0x0C, 0x0D, 0x0E, 0x0F,
	}

	b2 := []byte{
		0x10, 0x11, 0x12, 0x13,
		0x14, 0x15, 0x16, 0x17,
		0x18, 0x19, 0x1A, 0x1B,
		0x1C, 0x1D, 0x1E, 0x1F,
	}

	instructions.PushBytesAndPack(b1)
	instructions.PushBytesAndPack(b2)
	instructions.Halt()

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
