package vm

import "C"
import (
	"fmt"
	"github.com/andantan/hybrid-vm/ffi"
)

type VM struct {
	Ptr ffi.VmPtr
}

func NewVM(stackSize int, inst Instructions) (*VM, error) {
	if stackSize < 1 {
		return nil, fmt.Errorf("stack size must be at least 1")
	}

	if len(inst) == 0 {
		return nil, fmt.Errorf("empty instructions")
	}

	cInsts := inst.ToFFIOperationSlice()
	vmPtr, err := ffi.CreateVM(stackSize, cInsts)

	if err != nil {
		return nil, err
	}

	return &VM{
		Ptr: vmPtr,
	}, nil
}

func (vm *VM) Run() (any, error) {
	var result ffi.Result

	defer func() {
		result.Free()
		vm.Free()
	}()

	result = ffi.RunVM(vm.Ptr)

	if result.IsError {
		switch result.ErrorCode {
		case 0:
			return nil, fmt.Errorf("stack underflow")
		case 1:
			return nil, fmt.Errorf("stack overflow")
		case 2:
			return nil, fmt.Errorf("invalid type for operation")
		case 3:
			return nil, fmt.Errorf("division by zero")
		default:
			return nil, fmt.Errorf("unknown error (code: %d)", result.ErrorCode)
		}
	}

	var v any

	if result.IsFloat {
		v = result.FloatValue
	} else if result.IsByte {
		v = result.ByteValue
	} else if result.IsByteArray {
		v = result.ByteArrayValue[:]
	} else if result.IsBool {
		v = result.BoolValue
	} else {
		v = result.IntValue
	}

	// defer result.Free()

	return v, nil
}

func (vm *VM) Free() {
	ffi.FreeVm(vm.Ptr)
}
