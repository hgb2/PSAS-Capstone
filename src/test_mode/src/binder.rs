// binder.rs
//
//
extern crate libc;
mod wrapper;

pub fn init() -> *mut wrapper::FDM {

    //placeholder
    println!("test mode binder init");
    
    unsafe{
        let fdm = wrapper::fdm_create();
        return fdm;
    }
}

pub fn step(fdm: *mut wrapper::FDM){
    //placeholder 
    println!("test mode binder step");
}

pub fn close(fdm: *mut wrapper::FDM){
    //placeholder
    println!("test mode binder close");

//left for backwards compatibility.  should be deleted    
pub fn loopdata(fdm: &*mut libc::c_void) {
    println!("please replace call to binder::loopback with call to binder::step");
}

//left for backwards compatibility.  should be deleted    
pub fn terminate(fdm: &*mut libc::c_void) {
    println!("please replace call to binder::terminate with call to binder::close");
}
