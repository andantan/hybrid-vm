#ifndef STACK_VM_H
#define STACK_VM_H

#include <stdint.h>

#include "./stack_vm_op.h"
#include "./stack_vm_result.h"

extern void* create_vm(
    size_t stack_size,
    Operation* instruction_ptr,
    size_t instruction_len
);
extern VMResult run_vm(void* vm_ptr);
extern void free_byte_array(uint8_t* ptr, size_t len, size_t capacity);
extern void free_vm(void* vm_ptr);

#endif // STACK_VM_H