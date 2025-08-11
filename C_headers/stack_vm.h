#ifndef STACK_VM_H
#define STACK_VM_H

#include <stdint.h>
#include <stdbool.h>

#define PUSH 0
#define ADD 1
#define HALT 2

typedef struct {
    int32_t kind;
    int32_t val;
} C_OpCode;

typedef struct {
    int32_t result_code;
    bool is_error;
} OpResult;

extern void* create_vm(C_OpCode* instruction_ptr, size_t instruction_len);
extern OpResult run_vm(void* vm_ptr);
extern void free_vm(void* vm_ptr);

#endif // STACK_VM_H