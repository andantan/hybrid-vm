#ifndef STACK_VM_RESULT
#define STACK_VM_RESULT

#include <stdint.h>
#include <stdbool.h>

typedef struct {
    int32_t result;
    bool is_error;
} OpResult;

#endif // STACK_VM_RESULT