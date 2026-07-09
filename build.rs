fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = slint_build::CompilerConfiguration::new()
        .with_bundled_translations(format!("{}/lang", manifest_dir));
    slint_build::compile_with_config("ui/app.slint", config).expect("Slint build failed");
}
