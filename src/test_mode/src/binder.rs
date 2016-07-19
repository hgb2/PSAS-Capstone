//binder.rs
//
//
extern crate libc;
//mod wrapper;

//placeholder functions
pub fn binder_init()->*mut wrapper::FDM{
    println!("binder init");
    
    unsafe{
        let fdm = wrapper::fdm_create();
        return fdm;
    }    
}

pub fn binder_step(fdm: *mut wrapper::FDM ){
    println!("binder step");
}

pub fn binder_close(fdm: *mut wrapper::FDM ){
    println!("binder close");
}
