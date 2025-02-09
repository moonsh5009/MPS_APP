use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let source_dir = Path::new("obj");
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap().join(Path::new("obj"));

    // 디렉토리를 재귀적으로 복사
    fs::create_dir_all(&target_dir).unwrap();
    copy_dir_recursive(source_dir, &target_dir).expect("Failed to copy directory");

    println!("cargo:rerun-if-changed=config");
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();

        let target_path = dst.join(path.strip_prefix(src).unwrap());
        if path.is_dir() {
            fs::create_dir_all(&target_path)?;
            copy_dir_recursive(&path, &target_path)?;
        } else {
            fs::copy(&path, &target_path)?;
        }
    }

    Ok(())
}