fn main() {
    println!("cargo:rustc-link-search=src");
    println!("cargo:rustc-link-lib=static=_pos");
}
