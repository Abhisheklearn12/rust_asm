# x86-64 AT&T syntax
.section .text
.globl asm_multiply
.type asm_multiply, @function

asm_multiply:
    # Parameters: rdi = a, rsi = b
    # Return: rax = a * b
    movq %rdi, %rax
    imulq %rsi, %rax
    ret

.size asm_multiply, .-asm_multiply
