//wrapper.rs
//
//
extern crate libc;

use wrapper;
#[link(name = "stdc++")]
#[link(name = "JSBSim")]
#[link(name = "wrapper", kind = "static")]
extern "C" {
  
    //jsbsim constructor
    //JSBSim::FGFDMExec* fdm_create();
    pub fn fdm_create()->*mut libc::c_void;
    
    //jsbsim destructor
    //void fdm_close(JSBSim::FGFDMExec *fdm);
    pub fn fdm_close(fdm: *mut libc::c_void);
    
    //test functions
    //note that the original returns string
    //string GetVersion()
    //std::string fdm_get_version(JSBSim::FGFDMExec *fdm);
    pub fn fdm_get_version(fdm: *mut libc::c_void);
}

pub fn wrapper_init(){
    unsafe {
    let fdm = fdm_create();
    fdm_get_version(fdm);
    fdm_close(fdm);
    }
}