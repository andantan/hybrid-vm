package ffi

/*
   #cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
   #include "../C_headers/stack_vm.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

type VmPtr unsafe.Pointer

func CreateVM(stackSize int, insts []Operation) (VmPtr, error) {
	cStackSize := C.size_t(stackSize)
	cInstPtr := (*C.Operation)(unsafe.Pointer(&insts[0]))
	instLen := C.size_t(len(insts))

	vmPtr := C.create_vm(cStackSize, cInstPtr, instLen)

	if vmPtr == nil {
		return nil, fmt.Errorf("failed to create VM in Rust")
	}

	return VmPtr(vmPtr), nil
}

func RunVM(vmPtr VmPtr) Result {
	VMResult := C.run_vm(unsafe.Pointer(vmPtr))

	return NewResult(VMResult)
}

func FreeVm(vmPtr VmPtr) {
	C.free_vm(unsafe.Pointer(vmPtr))
}
