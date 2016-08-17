//wrapper.rs
//
//
extern crate libc;
use std;

//type: FGFDMExec
pub enum FDM{}
static mut fdm: *mut FDM = 0 as *mut FDM;

//state variables
static mut cw: u8 = 0;                  //clockwise:  0=off.  1=on.
static mut ccw: u8 = 0;                 //counter clockwise
static mut gyro_x: f32 = 0.0;           //gyro readings
static mut gyro_y: f32 = 0.0;
static mut gyro_z: f32 = 0.0;

//instantiates & initializes a flight dynamics model
//sets the environmental variables to jsbsim defaults
pub fn wrapper_init(){
    unsafe{
        //create a new fdm
        fdm = fdm_create();

        //set base jsbsim scripts folder
        let jsbsim_root_dir = std::ffi::CString::new("jsbsim/").unwrap();
        fdm_set_root_dir(fdm, jsbsim_root_dir.as_ptr());

        //set aircraft folder
        let apresult: bool;
        let jsbsim_aircraft_path = std::ffi::CString::new("aircraft").unwrap();
        apresult = fdm_set_aircraft_path(fdm, jsbsim_aircraft_path.as_ptr());

        //set engine folder
        let epresult: bool;
        let jsbsim_engine_path = std::ffi::CString::new("engine").unwrap();
        epresult = fdm_set_engine_path(fdm, jsbsim_engine_path.as_ptr());

        //set systems folder
        let spresult: bool;
        let jsbsim_systems_path = std::ffi::CString::new("systems").unwrap();
        spresult = fdm_set_systems_path(fdm, jsbsim_systems_path.as_ptr());

        //load script prep
        let script_name = std::ffi::CString::new("run.xml").unwrap();
        let delta_t: f64 = 0.0;
        let init_file = std::ffi::CString::new("").unwrap();
        let lsresult: bool;

        //load script
        lsresult = fdm_load_script(fdm, script_name.as_ptr(), delta_t, init_file.as_ptr());

        //run initial
        let icresult: bool;
        icresult = fdm_run_ic(fdm);
    }
}

//sends data to jsbsim.  iterates fdm.  gets response from jsbsim.
//development:  update state variable
pub fn send_to_jsbsim(newcw: u8, newccw: u8)->bool{
    let runresult: bool;
    unsafe{
        // This causes segfault
        runresult = fdm_run(fdm);

        //indicator for development
        gyro_x = gyro_x + 1.0;
        gyro_y = gyro_y + 2.0;
        gyro_z = gyro_z + 3.0;
    }
    return runresult;
}

//returns current state variable
pub fn get_from_jsbsim()->(f32, f32, f32){
    unsafe{
        return (gyro_x, gyro_y, gyro_z);
    }
}

//development:  jsbsim destructor
pub fn wrapper_close(){
    println!("binder close");
}

#[link(name = "stdc++")]
#[link(name = "JSBSim")]
#[link(name = "wrapper", kind = "static")]
//rust wrapper definitions using c abi
extern "C" {
    //jsbsim constructor
    fn fdm_create()->*mut FDM;

    //jsbsim destructor
    fn fdm_close(fdm: *mut FDM);

    //functions
    fn fdm_run(fdm: *mut FDM)->bool;
    fn fdm_run_ic(fdm: *mut FDM)->bool;
    fn fdm_load_script(fdm: *mut FDM, script_name: *const libc::c_char, delta_t: f64, init_file: *const libc::c_char)->bool;
    fn fdm_set_aircraft_path(fdm: *mut FDM, aircraft_path: *const libc::c_char)->bool;
    fn fdm_set_engine_path(fdm: *mut FDM, engine_path: *const libc::c_char)->bool;
    fn fdm_set_systems_path(fdm: *mut FDM, systems_path: *const libc::c_char)->bool;
    fn fdm_set_root_dir(fdm: *mut FDM, root_dir: *const libc::c_char);
}
