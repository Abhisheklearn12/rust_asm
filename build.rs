fn main() {
    // Compile Assembly files
    cc::Build::new()
        .file("src/asm/add.S")
        .file("src/asm/multiply.S")
        .file("src/asm/factorial.S")
        .flag("-x")
        .flag("assembler")
        .compile("asm_funcs");

    // Tell cargo to rerun if files change
    println!("cargo:rerun-if-changed=src/asm/");
}
