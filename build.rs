use cmake;

fn main() {
    let dst = cmake::Config::new(::std::env::var("DV_DIR").unwrap())
        .cxxflag("-std=c++17")
        .define("DIFFERVOID_BUILD_C_INTERFACE", "ON")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=differvoid");
}
