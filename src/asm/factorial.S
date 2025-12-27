# Recursive factorial in assembly
.section .text
.globl asm_factorial
.type asm_factorial, @function

asm_factorial:
    # Parameter: rdi = n
    # Return: rax = n!

    cmpq $1, %rdi           # if n <= 1
    jle .base_case          # return 1

    pushq %rdi              # Save n
    decq %rdi               # n - 1
    call asm_factorial      # factorial(n-1)
    popq %rdi               # Restore n
    imulq %rdi, %rax        # n * factorial(n-1)
    ret

.base_case:
    movq $1, %rax
    ret

.size asm_factorial, .-asm_factorial
