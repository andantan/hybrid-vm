#ifndef STACK_VM_H
#define STACK_VM_H

#include <stdint.h>

#include "./stack_vm_op.h"
#include "./stack_vm_result.h"

extern void* create_vm(C_OpCode* instruction_ptr, size_t instruction_len);
extern OpResult run_vm(void* vm_ptr);
extern void free_vm(void* vm_ptr);

#endif // STACK_VM_H