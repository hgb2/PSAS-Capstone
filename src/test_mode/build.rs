extern crate cmake;

fn main() {
    let dst = cmake::Config::new("src").build();
    println!("cargo:rustc-link-search=src");
    println!("cargo:rustc-link-search=native={}", dst.display());
}
