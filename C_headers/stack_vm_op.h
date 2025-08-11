#ifndef STACK_VM_OP
#define STACK_VM_OP

#include <stdint.h>

#define PUSH 0
#define ADD 1
#define HALT 2

typedef struct {
    int32_t kind;
    int32_t val;
} C_OpCode;

#endif // STACK_VM_OP