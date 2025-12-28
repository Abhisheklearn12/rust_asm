//! # Rust + x86-64 Assembly
//!
//! This library demonstrates integration of Rust with assembly.

// ASSEMBLY FUNCTION FFI DECLARATIONS

unsafe extern "C" {
    pub fn asm_add(a: i64, b: i64) -> i64;
    pub fn asm_multiply(a: i64, b: i64) -> i64;
    pub fn asm_factorial(n: i64) -> i64;

    // SIMD Addition using SSE2
    pub fn asm_simd_add_4(a: *const i32, b: *const i32, result: *mut i32);
}

// C FUNCTION FFI DECLARATIONS

unsafe extern "C" {
    // Math utilities
    pub fn c_power(base: i64, exponent: i32) -> i64;
    pub fn c_gcd(a: i64, b: i64) -> i64;
    pub fn c_lcm(a: i64, b: i64) -> i64;
    pub fn c_is_prime(n: i64) -> i32;
    pub fn c_fibonacci(n: i32) -> i64;
}

// SAFE RUST WRAPPERS - ASSEMBLY FUNCTIONS

pub fn add(a: i64, b: i64) -> i64 {
    unsafe { asm_add(a, b) }
}

pub fn multiply(a: i64, b: i64) -> i64 {
    unsafe { asm_multiply(a, b) }
}

pub fn factorial(n: i64) -> i64 {
    unsafe { asm_factorial(n) }
}

// Sage Rust WRASPPER for SIMD Addition using SSE2
pub fn simd_add_4(a: &[i32; 4], b: &[i32; 4]) -> [i32; 4] {
    let mut result = [0i32; 4];
    unsafe {
        asm_simd_add_4(a.as_ptr(), b.as_ptr(), result.as_mut_ptr());
    }
    result
}

// SAFE RUST WRAPPERS - C MATH FUNCTIONS

pub fn power(base: i64, exponent: i32) -> i64 {
    unsafe { c_power(base, exponent) }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    unsafe { c_gcd(a, b) }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    unsafe { c_lcm(a, b) }
}

pub fn is_prime(n: i64) -> bool {
    unsafe { c_is_prime(n) != 0 }
}

pub fn fibonacci(n: i32) -> i64 {
    unsafe { c_fibonacci(n) }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    // Assembly tests
    #[test]
    fn test_asm_add() {
        assert_eq!(add(5, 3), 8);
        assert_eq!(add(-5, 3), -2);
    }

    #[test]
    fn test_asm_multiply() {
        assert_eq!(multiply(5, 3), 15);
        assert_eq!(multiply(-5, 3), -15);
    }

    #[test]
    fn test_asm_factorial() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
    }

    // Unit test for SIMD Addition Using SSE2
    #[test]
    fn test_asm_simd() {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        let result = simd_add_4(&a, &b);
        assert_eq!(result, [6, 8, 10, 12]);
    }

    // C math tests
    #[test]
    fn test_c_power() {
        assert_eq!(power(2, 10), 1024);
        assert_eq!(power(5, 3), 125);
    }

    #[test]
    fn test_c_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(100, 50), 50);
    }

    #[test]
    fn test_c_lcm() {
        assert_eq!(lcm(12, 15), 60);
        assert_eq!(lcm(100, 50), 100);
    }

    #[test]
    fn test_c_is_prime() {
        assert!(is_prime(17));
        assert!(!is_prime(18));
        assert!(is_prime(2));
    }

    #[test]
    fn test_c_fibonacci() {
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(15), 610);
    }
}
