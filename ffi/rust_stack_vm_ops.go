package ffi

/*
   #cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
   #include "../C_headers/stack_vm_op.h"
*/
import "C"
import "unsafe"

type COpCode struct {
	kind C.int32_t
	val  C.int32_t
}

func NewCOpCode(kind OperationCode, value int32) COpCode {
	return COpCode{
		kind: C.int32_t(kind),
		val:  C.int32_t(value),
	}
}

type VmPtr unsafe.Pointer
type OperationCode C.int32_t
type OperationValue C.int32_t

const (
	OpCodePush OperationCode = C.PUSH
	OpCodeAdd  OperationCode = C.ADD
	OpCodeHalt OperationCode = C.HALT
)
