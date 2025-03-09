fn main() {
    // This tells Cargo to rerun this script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}
