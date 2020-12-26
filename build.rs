fn main() {
    println!("cargo:rustc-link-lib=framework=Hypervisor");
    println!("link-arg=-mmacosx-version-min=11.0");
}