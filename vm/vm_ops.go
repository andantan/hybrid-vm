package vm

import "github.com/andantan/hybrid-vm/ffi"

type Instructions []OpCode

func NewInstructions() Instructions {
	return make(Instructions, 0)
}

func (ins *Instructions) Push(op OpCode) {
	*ins = append(*ins, op)
}

type OpCode struct {
	kind ffi.OperationCode
	val  int32
}

func OpPush(value int32) OpCode {
	return OpCode{
		kind: ffi.OpCodePush,
		val:  value,
	}
}

func OpAdd() OpCode {
	return OpCode{
		kind: ffi.OpCodeAdd,
		val:  0,
	}
}

func OpHalt() OpCode {
	return OpCode{
		kind: ffi.OpCodeHalt,
		val:  0,
	}
}
