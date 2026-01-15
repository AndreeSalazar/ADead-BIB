// Build script para crear imagen UEFI con bootloader 0.11
use std::path::PathBuf;

fn main() {
    let kernel_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join("x86_64-unknown-none")
        .join("release")
        .join("fastos-kernel");

    let uefi_path = kernel_path.with_extension("efi");
    let bios_path = kernel_path.with_extension("img");

    // Crear imagen UEFI
    let uefi_builder = bootloader::UefiBoot::new(&kernel_path);
    uefi_builder.create_disk_image(&uefi_path).unwrap();

    // Crear imagen BIOS
    let bios_builder = bootloader::BiosBoot::new(&kernel_path);
    bios_builder.create_disk_image(&bios_path).unwrap();

    println!("cargo:rerun-if-changed=kernel/");
}
