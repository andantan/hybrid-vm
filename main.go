package main

/*
#cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
#include "./C_headers/stack_vm.h"
*/
import "C"

import (
	"fmt"
	"os"
	"unsafe"
)

type Opcode struct {
	kind int32
	val  int32
}

func main() {
	instructions := []Opcode{
		{kind: C.PUSH, val: 1},
		{kind: C.PUSH, val: 3},
		{kind: C.ADD, val: 0},
		{kind: C.HALT, val: 0},
	}

	instructionPtr := unsafe.Pointer(&instructions[0])
	instructionLen := C.size_t(len(instructions))

	vmPtr := C.create_vm((*C.C_OpCode)(instructionPtr), instructionLen)

	if vmPtr == nil {
		fmt.Println("Error: Failed to create VM")
		os.Exit(1)
	}

	result := C.run_vm(vmPtr)

	if result.is_error {
		fmt.Println("Error occurred during VM execution")
	} else {
		fmt.Printf("VM execution successful. Result: %d\n", int32(result.result_code))
	}

	C.free_vm(vmPtr)
}
