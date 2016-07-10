//binder.rs
//
//
#[link(name = "stdc++")]
#[link(name = "JSBSim")]
#[link(name = "SimWrapper", kind = "static")]
extern "C" {
	pub fn wrapper_init();
	pub fn wrapper_update();
	pub fn wrapper_terminate();
}

pub fn init(){

	//init binder
	println!("test mode binder init");
	
	//initialize wrapper
	unsafe{
		wrapper_init();
    }	
	
	//initialize jsbsim (incl. script)
	
}

pub fn loopdata(){
	
	//binder 
	println!("test mode binder loopdata");
	
	//call wrapper update
	unsafe{
		wrapper_update();
	}
	
	//call jsbsim
	
}

pub fn terminate(){

	//binder close
	println!("test mode binder close.");

	//close wrapper
	unsafe{
		wrapper_terminate();
	}
	
	//close jsbsim
}