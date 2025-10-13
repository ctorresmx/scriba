use std::process::Command;

fn main() {
    install_npm_packages();
    build_tailwind_css();
}

fn install_npm_packages() {
    let npm_binary = "npm";

    let output = Command::new(npm_binary)
        .args(["install"])
        .output()
        .expect("Failed to execute NPM");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("NPM install failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        println!("NPM output: {}", stdout);
    }

    println!("NPM install was successful");
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
