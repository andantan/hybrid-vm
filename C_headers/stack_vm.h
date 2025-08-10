#ifndef STACK_VM_H
#define STACK_VM_H

#include <stdint.h>
#include <stdbool.h>

typedef struct {
    int32_t result_code;
    bool is_error;
} OpResult;

extern OpResult run_vm(void* vm_ptr);
extern void* create_vm();
extern void free_vm(void* vm_ptr);

#endif