//extern crate cmake;
extern crate gcc;

fn main() {
    //previous version:  cmake
    //let dst = cmake::Config::new("src").build();
    //println!("cargo:rustc-link-search=src");
    //println!("cargo:rustc-link-search=native={}", dst.display());
    
    //test version:  gcc
    gcc::Config::new().cpp(true).file("src/wrapper.cpp").include("/usr/local/include/JSBSim/").compile("libwrapper.a");
        
}
