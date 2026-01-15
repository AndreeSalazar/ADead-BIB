// Script para crear imagen booteable de FastOS
// Ejecutar con: cargo run --package build_image

use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from("target/x86_64-unknown-none/release");
    let kernel = out_dir.join("fastos-kernel");
    
    // Primero compilar el kernel
    println!("Compilando kernel...");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .expect("Failed to build kernel");
    
    if !status.success() {
        panic!("Kernel build failed");
    }
    
    println!("Kernel compilado en: {:?}", kernel);
    
    // Crear imagen BIOS
    let bios_image = out_dir.join("fastos-bios.img");
    println!("Creando imagen BIOS: {:?}", bios_image);
    
    let bios_boot = bootloader::BiosBoot::new(&kernel);
    bios_boot.create_disk_image(&bios_image).expect("Failed to create BIOS image");
    
    println!("Imagen BIOS creada exitosamente!");
    println!("Ejecutar con: qemu-system-x86_64 -drive format=raw,file={:?}", bios_image);
}
