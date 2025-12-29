fn main() {
    // Compile Assembly files
    cc::Build::new()
        .file("src/asm/add.S")
        .file("src/asm/multiply.S")
        .file("src/asm/factorial.S")
        .file("src/asm/simd_add.S")
        .flag("-x")
        .flag("assembler")
        .compile("asm_funcs");

    // Compile C files
    cc::Build::new()
        .file("src/c/math_utils.c")
        .file("src/c/array_ops.c")
        .include("src/c")
        .opt_level(3)
        .flag("-Wall")
        .flag("-Wextra")
        .compile("c_funcs");

    // Tell cargo to rerun if files change
    println!("cargo:rerun-if-changed=src/asm/");
    println!("cargo:rerun-if-changed=src/c/");
}
