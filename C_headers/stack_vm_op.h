#ifndef STACK_VM_OP
#define STACK_VM_OP

#define OP_HALT       0
#define OP_PUSHINT    1
#define OP_PUSHFLOAT  2
#define OP_PUSHBYTE   3
#define OP_POP        4
#define OP_ADD        5
#define OP_SUB        6
#define OP_MUL        7
#define OP_DIV        8
#define OP_EQ         9
#define OP_LT         10
#define OP_GT         11
#define OP_LTE        12
#define OP_GTE        13

#include <stdint.h>

typedef union {
    int32_t int_val;
    float   float_val;
    uint8_t byte_val;
} OperationValue;

typedef struct {
    uint8_t kind;
    OperationValue val;
} Operation;

#endif // STACK_VM_OP