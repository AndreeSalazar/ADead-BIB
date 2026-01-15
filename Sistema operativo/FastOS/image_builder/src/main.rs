use std::path::PathBuf;
use std::env;

fn main() {
    // Obtener directorio del proyecto FastOS
    let fastos_dir = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    
    let kernel_path = fastos_dir
        .join("target")
        .join("x86_64-unknown-none")
        .join("release")
        .join("fastos-kernel");
    
    let bios_image = fastos_dir
        .join("target")
        .join("x86_64-unknown-none")
        .join("release")
        .join("fastos-bios.img");
    
    println!("FastOS Image Builder");
    println!("====================");
    println!("Kernel: {:?}", kernel_path);
    println!("Output: {:?}", bios_image);
    
    if !kernel_path.exists() {
        eprintln!("ERROR: Kernel no encontrado!");
        eprintln!("Ejecuta primero: cargo build --release");
        std::process::exit(1);
    }
    
    println!("\nCreando imagen BIOS...");
    
    let bios_boot = bootloader::BiosBoot::new(&kernel_path);
    bios_boot.create_disk_image(&bios_image).expect("Failed to create BIOS image");
    
    println!("Imagen creada exitosamente!");
    println!("\nPara ejecutar:");
    println!("qemu-system-x86_64 -drive format=raw,file={:?} -m 256M", bios_image);
}
