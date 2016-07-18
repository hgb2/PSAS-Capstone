//wrapper.rs
//
//
extern crate libc;

//type:  FGFDMExec
pub enum FDM{}

//verify that these links are needed
#[link(name = "stdc++")]
#[link(name = "JSBSim")]
#[link(name = "wrapper", kind = "static")]
extern "C"{
	//wrapper test functions (development only)
	pub fn wrapper_test();
	pub fn wrapper_init();
	pub fn wrapper_step();
	pub fn wrapper_close();
	
	//jsbsim constructor
	pub fn fdm_create()->*mut FDM;
	
	//jsbsim destructor
	pub fn fdm_close(fdm: *mut FDM);
	
	//jsbsim functions
	//these are some of the public functions of jsbsim's FGFDMExec class
	//note that these MUST be compatible with the function defintions 
	//that are listed in wrapper.h & implemented in wrapper.cpp
	pub fn fdm_run(fdm: *mut FDM)->bool;
	pub fn fdm_run_ic(fdm: *mut FDM)->bool;
}
