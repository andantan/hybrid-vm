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

func CreateVM(insts []COpCode) (VmPtr, error) {
	cInstPtr := (*C.C_OpCode)(unsafe.Pointer(&insts[0]))
	instLen := C.size_t(len(insts))

	vmPtr := C.create_vm(cInstPtr, instLen)

	if vmPtr == nil {
		return nil, fmt.Errorf("failed to create VM in Rust")
	}

	return VmPtr(vmPtr), nil
}

func RunVM(vmPtr VmPtr) OpResult {
	cOpResult := C.run_vm(unsafe.Pointer(vmPtr))

	return NewOpResult(cOpResult)
}

func FreeVm(vmPtr VmPtr) {
	C.free_vm(unsafe.Pointer(vmPtr))
}
