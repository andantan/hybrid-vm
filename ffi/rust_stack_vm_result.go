package ffi

/*
   #cgo LDFLAGS: -L${SRCDIR}/rust_stack_vm/target/release -lrust_stack_vm
   #include "../C_headers/stack_vm.h"
*/
import "C"
import "unsafe"

const (
	VMResultTagInteger   = C.VM_RESULT_INTEGER
	VMResultTagFloat     = C.VM_RESULT_FLOAT
	VMResultTagByteArray = C.VM_RESULT_BYTE_ARRAY
	VMResultTagError     = C.VM_RESULT_ERROR
)

type ByteArrayPtr = C.ByteArrayPtr

type Result struct {
	IsError        bool
	IsFloat        bool
	IsByteArray    bool
	IntValue       int32
	FloatValue     float32
	ByteArrayValue []byte
	ErrorCode      int32
	ByteArrayPtr   ByteArrayPtr
}

func NewResult(r C.VMResult) Result {
	switch r.tag {
	case VMResultTagInteger:
		intValue := *(*int32)(unsafe.Pointer(&r.value))
		return Result{
			IsError:  false,
			IsFloat:  false,
			IntValue: intValue,
		}

	case VMResultTagFloat:
		floatValue := *(*float32)(unsafe.Pointer(&r.value))
		return Result{
			IsError:    false,
			IsFloat:    true,
			FloatValue: floatValue,
		}

	case VMResultTagByteArray:
		bytesPtr := *(*ByteArrayPtr)(unsafe.Pointer(&r.value))
		goSlice := C.GoBytes(unsafe.Pointer(bytesPtr.ptr), C.int(bytesPtr.len))

		return Result{
			IsError:        false,
			IsByteArray:    true,
			ByteArrayValue: goSlice,
			ByteArrayPtr:   bytesPtr,
		}

	case VMResultTagError:
		fallthrough

	default:
		errorCode := *(*int32)(unsafe.Pointer(&r.value))
		return Result{
			IsError:   true,
			ErrorCode: errorCode,
		}
	}
}

func (r *Result) Free() {
	if r.IsByteArray && r.ByteArrayPtr.ptr != nil {
		arrPtr := r.ByteArrayPtr.ptr
		arrLen := C.size_t(r.ByteArrayPtr.len)
		arrCap := C.size_t(r.ByteArrayPtr.capacity)

		C.free_byte_array(arrPtr, arrLen, arrCap)

		r.ByteArrayPtr.ptr = nil
	}
}
