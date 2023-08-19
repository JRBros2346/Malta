fn main() {
    println!("cargo:rerun-if-changed=ui");
    slint_build::compile_with_config(
        "ui/main.slint",
        slint_build::CompilerConfiguration::new().with_style(String::from("material")),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
