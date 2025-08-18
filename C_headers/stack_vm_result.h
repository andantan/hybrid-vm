#ifndef STACK_VM_RESULT
#define STACK_VM_RESULT

#include <stdint.h>
#include <stdbool.h>

typedef enum {
    VM_RESULT_INTEGER,
    VM_RESULT_FLOAT,
    VM_RESULT_BYTE,
    VM_RESULT_BYTE_ARRAY,
    VM_RESULT_BOOL,
    VM_RESULT_ERROR,
} VMResultTag;

typedef struct {
    uint8_t*  ptr;
    size_t    len;
    size_t    capacity;
} ByteArrayPtr;

typedef union {
    int32_t         int_val;
    float           float_val;
    uint8_t         byte_val;
    ByteArrayPtr    byte_array_val;
    bool            bool_val;
} VMResultValue;

typedef struct {
    VMResultTag tag;
    VMResultValue value;
} VMResult;

#endif // STACK_VM_RESULT