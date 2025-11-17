//! Build script for VoxAI
//!
//! Handles platform-specific build configuration and Whisper.cpp compilation

use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    // Declare custom features for cfg checking
    println!("cargo:rustc-check-cfg=cfg(feature, values(\"cuda\", \"metal\", \"opencl\"))");

    // Print build information
    println!("cargo:warning=Building for {}-{}", target_os, target_arch);

    // Platform-specific configurations
    match target_os.as_str() {
        "windows" => configure_windows(),
        "macos" => configure_macos(),
        "linux" => configure_linux(),
        _ => {}
    }

    // Architecture-specific configurations
    match target_arch.as_str() {
        "aarch64" => {
            println!("cargo:rustc-cfg=arm");
        }
        "x86_64" => {
            println!("cargo:rustc-cfg=x86_64");
        }
        _ => {}
    }
}

fn configure_windows() {
    println!("cargo:rustc-cfg=platform_windows");
    // Windows-specific build steps will go here
    // TODO: Link against Windows SDK for system tray and hotkeys
}

fn configure_macos() {
    println!("cargo:rustc-cfg=platform_macos");
    // macOS-specific build steps
    // TODO: Link against Cocoa and Metal frameworks
}

fn configure_linux() {
    println!("cargo:rustc-cfg=platform_linux");
    // Linux-specific build steps
    // TODO: Link against ALSA/PulseAudio for audio
}
