package ffi

/*
   #cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
   #include "../C_headers/stack_vm_result.h"
*/
import "C"
import "unsafe"

const (
	VMResultTagInteger = C.VM_RESULT_INTEGER
	VMResultTagFloat   = C.VM_RESULT_FLOAT
	VMResultTagError   = C.VM_RESULT_ERROR
)

type Result struct {
	IsError    bool
	IsFloat    bool
	IntValue   int32
	FloatValue float32
	ErrorCode  int32
}

func NewResult(cResult C.VMResult) Result {
	switch cResult.tag {
	case VMResultTagInteger:
		intValue := *(*int32)(unsafe.Pointer(&cResult.value))
		return Result{
			IsError:  false,
			IsFloat:  false,
			IntValue: intValue,
		}

	case VMResultTagFloat:
		floatValue := *(*float32)(unsafe.Pointer(&cResult.value))
		return Result{
			IsError:    false,
			IsFloat:    true,
			FloatValue: floatValue,
		}

	case VMResultTagError:
		fallthrough

	default:
		errorCode := *(*int32)(unsafe.Pointer(&cResult.value))
		return Result{
			IsError:   true,
			ErrorCode: errorCode,
		}
	}
}
