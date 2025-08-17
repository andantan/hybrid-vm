#ifndef STACK_VM_OP
#define STACK_VM_OP

#define OP_HALT       0x00
#define OP_PUSHINT    0x01
#define OP_PUSHFLOAT  0x02
#define OP_PUSHBYTE   0x03
#define OP_PACK       0x04
#define OP_POP        0x05
#define OP_ADD        0x06
#define OP_SUB        0x07
#define OP_MUL        0x08
#define OP_DIV        0x09
#define OP_EQ         0x0A
#define OP_GT         0x0B
#define OP_GTE        0x0C
#define OP_LT         0x0D
#define OP_LTE        0x0E

#include <stdint.h>

typedef union {
    int32_t  int_val;
    uint32_t uint_val;
    float    float_val;
    uint8_t  byte_val;
} OperationValue;

typedef struct {
    uint8_t kind;
    OperationValue val;
} Operation;

#endif // STACK_VM_OP