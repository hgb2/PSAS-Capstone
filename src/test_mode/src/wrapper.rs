// wrapper.rs
//
//
extern crate libc;

// type:  FGFDMExec
pub enum FDM {}

// verify that these links are needed
#[link(name = "stdc++")]
#[link(name = "JSBSim")]
#[link(name = "wrapper", kind = "static")]
extern "C" {
    // wrapper test functions (development only)
    pub fn wrapper_test();

    // jsbsim constructor
    pub fn fdm_create() -> *mut FDM;

    // jsbsim destructor
    pub fn fdm_close(fdm: *mut FDM);

    // jsbsim functions
    // these are some of the public functions of jsbsim's FGFDMExec class
    // note that these MUST be compatible with the function defintions
    // that are listed in wrapper.h & implemented in wrapper.cpp
    pub fn fdm_run(fdm: *mut FDM) -> bool;
    pub fn fdm_run_ic(fdm: *mut FDM) -> bool;
    
    pub fn fdm_load_script(fdm: *mut FDM) -> bool;
}

// Binder functions
pub fn init() -> *mut FDM {

    // placeholder
    println!("test mode binder init");

    unsafe {
        let fdm = fdm_create();
        fdm_load_script(fdm);
        return fdm;
    }
}

pub fn step(fdm: *mut FDM) {
    // placeholder
    println!("test mode binder step");
}

pub fn close(fdm: *mut FDM) {
    // placeholder
    println!("test mode binder close");
}

