package vm

import (
	"github.com/andantan/hybrid-vm/ffi"
)

type OpCode struct {
	kind ffi.OperationCode
	val  any
}

func NewOpHalt() OpCode {
	return OpCode{
		kind: ffi.OpCodeHalt,
		val:  nil,
	}
}

func NewOpPushInt(value int32) OpCode {
	return OpCode{
		kind: ffi.OpCodePushInt,
		val:  value,
	}
}

func NewOpPushFloat(value float32) OpCode {
	return OpCode{
		kind: ffi.OpCodePushFloat,
		val:  value,
	}
}

func NewOpPushByte(value byte) OpCode {
	return OpCode{
		kind: ffi.OpCodePushByte,
		val:  int32(value),
	}
}

func NewOpPack(size uint32) OpCode {
	return OpCode{
		kind: ffi.OpCodePack,
		val:  size,
	}
}

func NewOpPop() OpCode {
	return OpCode{
		kind: ffi.OpCodePop,
		val:  nil,
	}
}

func NewOpAdd() OpCode {
	return OpCode{
		kind: ffi.OpCodeAdd,
		val:  nil,
	}
}

func NewOpSub() OpCode {
	return OpCode{
		kind: ffi.OpCodeSub,
		val:  nil,
	}
}

func NewOpMul() OpCode {
	return OpCode{
		kind: ffi.OpCodeMul,
		val:  nil,
	}
}

func NewOpDiv() OpCode {
	return OpCode{
		kind: ffi.OpCodeDiv,
		val:  nil,
	}
}

func NewOpEq() OpCode {
	return OpCode{
		kind: ffi.OpCodeEq,
		val:  nil,
	}
}

func NewOpGt() OpCode {
	return OpCode{
		kind: ffi.OpCodeGt,
		val:  nil,
	}
}

func NewOpGte() OpCode {
	return OpCode{
		kind: ffi.OpCodeGte,
		val:  nil,
	}
}

func NewOpLt() OpCode {
	return OpCode{
		kind: ffi.OpCodeLt,
		val:  nil,
	}
}

func NewOpLte() OpCode {
	return OpCode{
		kind: ffi.OpCodeLte,
		val:  nil,
	}
}
