#include "math_utils.h"

int64_t c_power(int64_t base, int32_t exponent) {
    if (exponent == 0) return 1;

    int64_t result = 1;
    int64_t current_base = base;
    int32_t exp = exponent;

    // Fast exponentiation by squaring
    while (exp > 0) {
        if (exp & 1) {
            result *= current_base;
        }
        current_base *= current_base;
        exp >>= 1;
    }

    return result;
}

int64_t c_gcd(int64_t a, int64_t b) {
    // Euclidean algorithm
    if (a < 0) a = -a;
    if (b < 0) b = -b;

    while (b != 0) {
        int64_t temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

int64_t c_lcm(int64_t a, int64_t b) {
    if (a == 0 || b == 0) return 0;

    int64_t gcd = c_gcd(a, b);
    return (a / gcd) * b;
}

int c_is_prime(int64_t n) {
    if (n <= 1) return 0;
    if (n <= 3) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;

    for (int64_t i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0)
            return 0;
    }
    return 1;
}

int64_t c_fibonacci(int32_t n) {
    if (n <= 1) return n;

    int64_t prev = 0, curr = 1;

    for (int32_t i = 2; i <= n; i++) {
        int64_t next = prev + curr;
        prev = curr;
        curr = next;
    }

    return curr;
}
