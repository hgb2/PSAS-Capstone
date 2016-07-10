//binder.rs
//
//
#[link(name = "stdc++")]
#[link(name = "JSBSim")]
#[link(name = "SimWrapper", kind = "static")]

extern "C" {
	pub fn fdm_create() -> u64;
	pub fn fdm_get_version(fdm: &u64);
	pub fn fdm_close(fdm: &u64);
}

pub enum BinderError {
    JSBSimInitError,
}

pub fn init() -> u64 {

	//init binder
	println!("test mode binder init");

	let fdm: u64;	

	//initialize wrapper
	unsafe {
		fdm = fdm_create();
		fdm_get_version(&fdm);
		return fdm;
    	}	
	
	panic!("Unable to initialized JSBSim wrapper");
	fdm = 0;
	return fdm;
}

pub fn loopdata() {
	
	//binder 
	println!("test mode binder loopdata");
	//call jsbsim
	
}

pub fn terminate(fdm: &u64) {

	//binder close
	println!("test mode binder close.");

	//close wrapper
	unsafe {
		fdm_close(fdm);
	}
	
	//close jsbsim
}
