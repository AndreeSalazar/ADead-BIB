use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap();
    
    let kernel_path = workspace_dir
        .join("target")
        .join("x86_64-unknown-none")
        .join("release")
        .join("fastos-kernel");
    
    let out_dir = workspace_dir.join("target");
    let bios_image = out_dir.join("fastos-bios.img");
    
    println!("FastOS Boot Image Builder");
    println!("Kernel: {}", kernel_path.display());
    
    if !kernel_path.exists() {
        eprintln!("ERROR: Kernel no encontrado!");
        eprintln!("Ejecuta: cargo build --release -p fastos");
        std::process::exit(1);
    }
    
    println!("Creando imagen BIOS...");
    let bios = bootloader::BiosBoot::new(&kernel_path);
    bios.create_disk_image(&bios_image).expect("Failed to create BIOS image");
    
    println!("Imagen: {}", bios_image.display());
}
