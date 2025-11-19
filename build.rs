fn main() {
    slint_build::compile_with_config(
        "ui/app-window.slint",
        slint_build::CompilerConfiguration::new().with_style("native".to_string()),
    )
    .expect("Slint build failed");
}
