use std::{env, fs, path::Path};

fn main() {
    // Çıktı dizinini al
    let cargo_home = env::var("CARGO_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap();
        format!("{}/.cargo", home)
    });
    let bin_dir = Path::new(&cargo_home).join("bin");

    // Dil dosyalarını kopyala
    let locales_dir = Path::new("locales");
    if locales_dir.exists() {
        let target_locales = bin_dir.join("locales");
        copy_dir_recursive(locales_dir, &target_locales).unwrap();
        println!("cargo:rerun-if-changed=locales");
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
} 