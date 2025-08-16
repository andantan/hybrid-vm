package vm

import "github.com/andantan/hybrid-vm/ffi"

type Instructions []OpCode

func NewInstructions() Instructions {
	return make(Instructions, 0)
}

func (ins *Instructions) Push(op OpCode) {
	*ins = append(*ins, op)
}

func (ins *Instructions) ToOperationSlice() []ffi.Operation {
	cInsts := make([]ffi.Operation, len(*ins))

	for i, op := range *ins {
		cInsts[i] = ffi.NewOperation(op.kind, op.val)
	}

	return cInsts
}

type OpCode struct {
	kind ffi.OperationCode
	val  any
}

func OpHalt() OpCode {
	return OpCode{
		kind: ffi.OpCodeHalt,
		val:  nil,
	}
}

func OpPushInt(value int32) OpCode {
	return OpCode{
		kind: ffi.OpCodePushInt,
		val:  value,
	}
}

func OpPushFloat(value float32) OpCode {
	return OpCode{
		kind: ffi.OpCodePushFloat,
		val:  value,
	}
}

func OpPushByte(value byte) OpCode {
	return OpCode{
		kind: ffi.OpCodePushByte,
		val:  int32(value),
	}
}

func OpPop() OpCode {
	return OpCode{
		kind: ffi.OpCodePop,
		val:  nil,
	}
}

func OpAdd() OpCode {
	return OpCode{
		kind: ffi.OpCodeAdd,
		val:  nil,
	}
}

func OpSub() OpCode {
	return OpCode{
		kind: ffi.OpCodeSub,
		val:  nil,
	}
}

func OpMul() OpCode {
	return OpCode{
		kind: ffi.OpCodeMul,
		val:  nil,
	}
}

func OpDiv() OpCode {
	return OpCode{
		kind: ffi.OpCodeDiv,
		val:  nil,
	}
}

func OpEq() OpCode {
	return OpCode{
		kind: ffi.OpCodeEq,
		val:  nil,
	}
}

func OpLt() OpCode {
	return OpCode{
		kind: ffi.OpCodeLt,
		val:  nil,
	}
}

func OpLte() OpCode {
	return OpCode{
		kind: ffi.OpCodeLte,
		val:  nil,
	}
}

func OpGt() OpCode {
	return OpCode{
		kind: ffi.OpCodeGt,
		val:  nil,
	}
}

func OpGte() OpCode {
	return OpCode{
		kind: ffi.OpCodeGte,
		val:  nil,
	}
}
