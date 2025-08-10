package main

/*
#cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
#include "./C_headers/stack_vm.h"
*/
import "C"

import (
	"fmt"
	"os"
)

func main() {
	vmPtr := C.create_vm()
	if vmPtr == nil {
		fmt.Println("Error: Failed to create VM")
		os.Exit(1)
	}

	result := C.run_vm(vmPtr)

	if result.is_error {
		fmt.Println("Error occurred during VM execution!")
	} else {
		fmt.Printf("VM execution successful. Result: %d\n", int32(result.result_code))
	}

	C.free_vm(vmPtr)
}
