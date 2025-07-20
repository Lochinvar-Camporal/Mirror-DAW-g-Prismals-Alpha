fn main() {
    println!("cargo:rustc-link-search=native={}/Lib", std::env::var("VULKAN_SDK").unwrap());
    println!("cargo:rustc-link-lib=dylib=vulkan-1");
}
