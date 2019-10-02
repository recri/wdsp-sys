
fn main() {
    // Tell cargo to tell rustc to link the system wdsp shared library
    println!("cargo:rustc-link-lib=wdsp");
}
