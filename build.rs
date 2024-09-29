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
    let env_dv_dir = env::var("DV_DIR").unwrap();
    let target_dir = get_output_path();
    let dv_dir = Path::new(&env_dv_dir);
    let tt_dir = Path::new(&target_dir);

    if cfg!(windows) {
        let src = dv_dir.join("bin/differvoid.dll");
        let dst = tt_dir.join("differvoid.dll");
        fs::copy(&src, &dst).unwrap();
    }

    println!("cargo:rustc-link-search=native={}/lib", dv_dir.display());
    println!("cargo:rustc-link-lib=dylib=differvoid");
}
