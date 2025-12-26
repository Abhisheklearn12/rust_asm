use rust_asm_demo::*;

fn main() {
    println!("  Rust + x86-64 Assembly + C Integration Demonstration");

    // ASSEMBLY FUNCTIONS DEMO
    println!("ASSEMBLY FUNCTIONS:");
    println!("─────────────────────────────────────────────────────────────");

    println!("Basic operations:");
    println!("  add(42, 13) = {}", add(42, 13));
    println!("  multiply(7, 6) = {}", multiply(7, 6));
}
