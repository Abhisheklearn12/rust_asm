# SIMD vector addition using SSE2
.section .text
.globl asm_simd_add_4
.type asm_simd_add_4, @function

asm_simd_add_4:
    # Parameters: rdi = ptr to array a (4 i32s)
    #             rsi = ptr to array b (4 i32s)
    #             rdx = ptr to result array (4 i32s)
    # Adds 4 integers in parallel using SSE2

    movdqu (%rdi), %xmm0    # Load 4 i32s from array a
    movdqu (%rsi), %xmm1    # Load 4 i32s from array b
    paddd %xmm1, %xmm0      # Add the vectors
    movdqu %xmm0, (%rdx)    # Store result
    ret

.size asm_simd_add_4, .-asm_simd_add_4
