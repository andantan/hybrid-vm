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

	cInsts := inst.ToOperationSlice()
	vmPtr, err := ffi.CreateVM(stackSize, cInsts)

	if err != nil {
		return nil, err
	}

	return &VM{
		Ptr: vmPtr,
	}, nil
}

func (vm *VM) Run() (any, error) {
	defer vm.Free()

	result := ffi.RunVM(vm.Ptr)

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

	if result.IsFloat {
		return result.FloatValue, nil
	}

	return result.IntValue, nil
}

func (vm *VM) Free() {
	ffi.FreeVm(vm.Ptr)
}
