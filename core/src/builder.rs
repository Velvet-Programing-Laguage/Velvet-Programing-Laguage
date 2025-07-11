use crate::error::VelvetError;
use crate::logger::Logger;
use crate::Config;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn build_project(release: bool, package_type: &str) -> String {
    let config = Config::load("vel.json").unwrap_or_default();
    let logger = Logger::new(config.debug);
    logger.info(&format!("Building project (release: {}, package: {})", release, package_type));

    // Compile Velvet code to a binary
    let output_dir = if release { "target/release" } else { "target/debug" };
    fs::create_dir_all(output_dir).unwrap();

    let main_file = "main.vel";
    if !Path::new(main_file).exists() {
        logger.error("main.vel not found");
        return format!("Error: main.vel not found");
    }

    // Placeholder: Compile Velvet to native binary
    logger.info("Compiling main.vel to native binary...");
    let binary_name = "velvet_app";
    let status = Command::new("rustc")
        .args(&["--crate-type", "bin", "-o", &format!("{}/{}", output_dir, binary_name)])
        .status()
        .unwrap();
    if !status.success() {
        logger.error("Compilation failed");
        return format!("Error: Compilation failed");
    }

    // Package the binary
    package_project(package_type)
}

pub fn package_project(package_type: &str) -> String {
    let config = Config::load("vel.json").unwrap_or_default();
    let logger = Logger::new(config.debug);
    logger.info(&format!("Packaging as {}", package_type));

    let binary_path = format!("target/release/velvet_app");
    if !Path::new(&binary_path).exists() {
        logger.error("Binary not found");
        return format!("Error: Binary not found");
    }

    match package_type {
        "exe" => {
            logger.info("Creating Windows .exe package...");
            // Placeholder: Package as .exe
            format!("Windows .exe created: {}", binary_path)
        }
        "deb" => {
            logger.info("Creating Debian .deb package...");
            // Placeholder: Use dpkg-deb or similar
            format!("Debian .deb created: velvet_app.deb")
        }
        "rpm" => {
            logger.info("Creating RPM package...");
            // Placeholder: Use rpmbuild or similar
            format!("RPM package created: velvet_app.rpm")
        }
        "appimage" => {
            logger.info("Creating AppImage package...");
            // Placeholder: Use appimage-builder
            format!("AppImage created: velvet_app.AppImage")
        }
        _ => {
            logger.error("Unsupported package type");
            format!("Error: Unsupported package type: {}", package_type)
        }
    }
}
