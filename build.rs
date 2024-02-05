fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" || target_os == "ios" {
        println!("cargo:rustc-cfg=moltenvk");
    } else {
        println!("cargo:rustc-cfg=loader");
    }
}