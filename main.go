package main

/*
#cgo LDFLAGS: -L${SRCDIR}/rust_vm/target/release -lrust_vm

extern void* create_vm();
extern int run_vm(void* vm_ptr);
extern void free_vm(void* vm_ptr);
*/
import "C"

import (
	"fmt"
)

func main() {
	vmPtr := C.create_vm()
	result := C.run_vm(vmPtr)
	C.free_vm(vmPtr)

	fmt.Printf("Rust VM result: %d\n", int(result))
}
