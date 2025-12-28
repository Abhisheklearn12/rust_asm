use rust_asm_demo::*;

fn main() {
    println!("Rust + x86-64 Assembly + C Integration Demo");

    // ASSEMBLY FUNCTIONS DEMO
    println!("ASSEMBLY FUNCTIONS:");
    println!("Basic operations:");
    println!("  add(42, 13) = {}", add(42, 13));
    println!("  multiply(7, 6) = {}", multiply(7, 6));

    // Added "Factorial Function"
    println!("\nRecursive factorial:");
    println!("  5! = {}", factorial(5));
    println!("  10! = {}", factorial(10));

    // SIMD Addition using SSE2
    println!("\nSIMD vector addition:");
    let vec_a = [10, 20, 30, 40];
    let vec_b = [1, 2, 3, 4];
    let result = simd_add_4(&vec_a, &vec_b);
    println!("  {:?} + {:?} = {:?}", vec_a, vec_b, result);

    // C MATH FUNCTIONS DEMO
    println!("\nC MATH FUNCTIONS:");

    println!("Power function:");
    println!("  2^10 = {}", power(2, 10));
    println!("  5^3 = {}", power(5, 3));

    println!("\nGCD and LCM:");
    println!("  gcd(48, 18) = {}", gcd(48, 18));
    println!("  lcm(12, 15) = {}", lcm(12, 15));

    println!("\nPrime checking:");
    for n in [17, 18, 19, 20, 97] {
        println!("  is_prime({}) = {}", n, is_prime(n));
    }

    println!("\nFibonacci sequence:");
    print!("  First 15 numbers: ");
    for i in 0..15 {
        print!("{} ", fibonacci(i));
    }
    println!();
}
