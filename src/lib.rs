//! # Rust + x86-64 Assembly
//!
//! This library demonstrates integration of Rust with assembly.

// ASSEMBLY FUNCTION FFI DECLARATIONS

unsafe extern "C" {
    pub fn asm_add(a: i64, b: i64) -> i64;
    pub fn asm_multiply(a: i64, b: i64) -> i64;
    pub fn asm_factorial(n: i64) -> i64;
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
}
