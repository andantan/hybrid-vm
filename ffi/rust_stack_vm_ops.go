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

type Operation C.Operation
type OperationCode C.uint8_t
type OperationValue C.OperationValue

const (
	OpCodeHalt      OperationCode = C.OP_HALT
	OpCodePushInt   OperationCode = C.OP_PUSHINT
	OpCodePushFloat OperationCode = C.OP_PUSHFLOAT
	OpCodePushByte  OperationCode = C.OP_PUSHBYTE
	OpCodePack      OperationCode = C.OP_PACK
	OpCodePop       OperationCode = C.OP_POP
	OpCodeAdd       OperationCode = C.OP_ADD
	OpCodeSub       OperationCode = C.OP_SUB
	OpCodeMul       OperationCode = C.OP_MUL
	OpCodeDiv       OperationCode = C.OP_DIV
	OpCodeEq        OperationCode = C.OP_EQ
	OpCodeLt        OperationCode = C.OP_LT
	OpCodeLte       OperationCode = C.OP_LTE
	OpCodeGt        OperationCode = C.OP_GT
	OpCodeGte       OperationCode = C.OP_GTE
	OpCodeConcat    OperationCode = C.OP_CONCAT
)

func NewOperation(kind OperationCode, val any) Operation {
	var op Operation
	op.kind = C.uint8_t(kind)

	switch v := val.(type) {
	case int32:
		*(*C.int32_t)(unsafe.Pointer(&op.val)) = C.int32_t(v)

	case uint32:
		*(*C.uint32_t)(unsafe.Pointer(&op.val)) = C.uint32_t(v)

	case float32:
		*(*C.float)(unsafe.Pointer(&op.val)) = C.float(v)

	case uint8:
		*(*C.uint8_t)(unsafe.Pointer(&op.val)) = C.uint8_t(v)

	case nil:

	default:
		panic(fmt.Sprintf("unsupported value type for opcode: %T", v))
	}

	return op
}
