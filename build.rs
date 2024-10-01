use std::path::{Path, PathBuf};
use std::{env, fs};

fn get_output_path() -> PathBuf {
    /* <root or manifest path>/target/<profile>/ */
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn main() {
    println!("cargo:rerun-if-env-changed=DV_DIR");

    let env_dv_dir = env::var("DV_DIR").unwrap();
    let target_dir = get_output_path();
    let dv_dir = Path::new(&env_dv_dir);
    let tt_dir = Path::new(&target_dir);

    println!("DV_DIR: {}", dv_dir.display());
    println!("TARGET_DIR: {}", tt_dir.display());

    let (src, dst);

    if cfg!(windows) {
        src = dv_dir.join("bin/differvoid.dll");
        dst = tt_dir.join("differvoid.dll");
    } else if cfg!(unix) {
        src = dv_dir.join("lib/libdiffervoid.so");
        dst = tt_dir.join("libdiffervoid.so");
    } else {
        panic!("OS not supported.");
    }

    println!("cargo:rerun-if-changed={}", src.display());

    match fs::copy(&src, &dst) {
        Err(_) => {
            println!(
                "cargo:warning=Failed to copy {} to {}",
                src.display(),
                dst.display()
            );
        }
        _ => {}
    }

    println!("cargo:rustc-link-search=native={}/lib", dv_dir.display());
    println!("cargo:rustc-link-lib=dylib=differvoid");
}
