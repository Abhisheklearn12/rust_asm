#ifndef MATH_UTILS
#define MATH_UTILS
#include <stdint.h>

// Power function: base^exponent
int64_t c_power(int64_t base, int32_t exponent);

// Greatest common divisor
int64_t c_gcd(int64_t a, int64_t b);

// Least common multiple
int64_t c_lcm(int64_t a, int64_t b);

// Check if number is prime
int c_is_prime(int64_t n);

// Fibonacci number (iterative)
int64_t c_fibonacci(int32_t n);

#endif
