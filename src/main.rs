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

    // C ARRAY FUNCTIONS DEMO
    println!("\nC ARRAY FUNCTIONS:");

    let numbers = vec![15, 3, 8, 12, 6, 20, 1, 9];
    println!("Array: {:?}", numbers);
    println!("  Sum: {}", array_sum(&numbers));
    println!("  Max: {}", array_max(&numbers));
    println!("  Min: {}", array_min(&numbers));
    println!("  Average: {:.2}", array_average(&numbers));

    let sorted = vec![1, 3, 5, 7, 9, 11, 13, 15];
    println!("\nBinary search in sorted array: {:?}", sorted);
    println!("  Search for 7: {:?}", binary_search(&sorted, 7));
    println!("  Search for 10: {:?}", binary_search(&sorted, 10));

    let mut unsorted = vec![64, 34, 25, 12, 22, 11, 90];
    println!("\nBubble sort:");
    println!("  Before: {:?}", unsorted);
    bubble_sort(&mut unsorted);
    println!("  After:  {:?}", unsorted);

    // C STRING FUNCTIONS DEMO
    println!("\n C MATH FUNCTIONS:");
    println!("─────────────────────────────────────────────────────────────");

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

    // ========================================================================
    // C STRING FUNCTIONS DEMO
    // ========================================================================
    println!("\n C STRING FUNCTIONS:");

    let mut text = String::from("hello");
    println!("Original: \"{}\"", text);
    string_reverse(&mut text);
    println!("Reversed: \"{}\"", text);

    println!("\nPalindrome checking:");
    for word in ["racecar", "hello", "madam", "world"] {
        println!("  \"{}\" is palindrome: {}", word, is_palindrome(word));
    }

    let sample = "hello world";
    println!("\nCharacter counting in \"{}\":", sample);
    println!("  'l' appears {} times", count_char(sample, 'l'));
    println!("  'o' appears {} times", count_char(sample, 'o'));

    let mut upper = String::from("make me loud");
    println!("\nCase conversion:");
    println!("  Original: \"{}\"", upper);
    to_uppercase(&mut upper);
    println!("  Uppercase: \"{}\"", upper);
    to_lowercase(&mut upper);
    println!("  Lowercase: \"{}\"", upper);
}
