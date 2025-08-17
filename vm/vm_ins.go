package vm

import "github.com/andantan/hybrid-vm/ffi"

type Instructions []OpCode

func NewInstructions() Instructions {
	return make(Instructions, 0)
}

func (ins *Instructions) PushOperation(op OpCode) {
	*ins = append(*ins, op)
}

func (ins *Instructions) PushBytes(data []byte) {
	for _, b := range data {
		*ins = append(*ins, NewOpPushByte(b))
	}
}

func (ins *Instructions) PushBytesAndPack(data []byte) {
	ins.PushBytes(data)
	*ins = append(*ins, NewOpPack(uint32(len(data))))
}

func (ins *Instructions) Halt() {
	*ins = append(*ins, NewOpHalt())
}

func (ins *Instructions) ToFFIOperationSlice() []ffi.Operation {
	cInsts := make([]ffi.Operation, len(*ins))

	for i, op := range *ins {
		cInsts[i] = ffi.NewOperation(op.kind, op.val)
	}

	return cInsts
}
