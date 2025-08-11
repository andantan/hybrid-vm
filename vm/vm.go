package vm

import "C"
import (
	"fmt"
	"github.com/andantan/hybrid-vm/ffi"
)

type VM struct {
	Ptr ffi.VmPtr
}

func NewVM(inst Instructions) (*VM, error) {
	if len(inst) == 0 {
		return nil, fmt.Errorf("empty instructions")
	}

	cInsts := make([]ffi.COpCode, len(inst))

	for i, op := range inst {
		cInsts[i] = ffi.NewCOpCode(op.kind, op.val)
	}

	vmPtr, err := ffi.CreateVM(cInsts)

	if err != nil {
		return nil, err
	}

	return &VM{
		Ptr: vmPtr,
	}, nil
}
