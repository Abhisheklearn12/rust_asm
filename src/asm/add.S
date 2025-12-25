# x86-64 AT&T
.section .text
.globl asm_add
.type asm_add, @function

asm_add:
    # Parameters: rdi = a, rsi = b
    # Return: rax = a + b
    movq %rdi, %rax
    addq %rsi, %rax
    ret

.size asm_add, .-asm_add
