package ffi

/*
   #cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
   #include "../C_headers/stack_vm_result.h"
*/
import "C"

type OpResult struct {
	Result  int32
	IsError bool
}

func NewOpResult(cResult C.OpResult) OpResult {
	return OpResult{
		Result:  int32(cResult.result),
		IsError: bool(cResult.is_error),
	}
}
