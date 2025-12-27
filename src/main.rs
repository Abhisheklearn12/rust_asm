use rust_asm_demo::*;

fn main() {
    println!("Rust + x86-64 Assembly Integration Demo");

    // ASSEMBLY FUNCTIONS DEMO
    println!("ASSEMBLY FUNCTIONS:");
    println!("Basic operations:");
    println!("  add(42, 13) = {}", add(42, 13));
    println!("  multiply(7, 6) = {}", multiply(7, 6));

    // Added "Factorial Function"
    println!("\nRecursive factorial:");
    println!("  5! = {}", factorial(5));
    println!("  10! = {}", factorial(10));
}
