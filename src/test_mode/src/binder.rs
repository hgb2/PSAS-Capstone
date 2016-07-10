//binder.rs
//
//

extern crate libc;

#[link(name = "stdc++")]
#[link(name = "JSBSim")]
#[link(name = "SimWrapper", kind = "static")]

extern "C" {
	pub fn fdm_create() -> *mut libc::c_void;
	pub fn fdm_get_version(fdm: &*mut libc::c_void);
	pub fn fdm_close(fdm: &*mut libc::c_void);
}

pub fn init() -> *mut libc::c_void {

	//init binder
	println!("test mode binder init");

	let fdm: *mut libc::c_void;	

	//initialize wrapper
	unsafe {
		fdm = fdm_create();
		fdm_get_version(&fdm);
		return fdm;
    	}	
	
	panic!("Unable to initialized JSBSim wrapper");
}

pub fn loopdata() {
	
	//binder 
	println!("test mode binder loopdata");
	//call jsbsim
	
}

pub fn terminate(fdm: &*mut libc::c_void) {

	//binder close
	println!("test mode binder close.");

	//close wrapper
	unsafe {
		fdm_close(fdm);
	}
	
	//close jsbsim
}
