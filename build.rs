use std::process::Command;

fn main() {
    build_tailwind_css();
}

fn build_tailwind_css() {
    let tailwind_binary = "./tailwindcss";

    let output = Command::new(tailwind_binary)
        .args([
            "-i",
            "static/styles.css",
            "-o",
            "static/rust_styles.css",
            "-m",
        ])
        .output()
        .expect("Failed to execute Tailwind CLI");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Tailwind CSS build failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        println!("Tailwind output: {}", stdout);
    }

    println!("Tailwind CSS built successfully");
}
