///////////////////////////////////////////////////////////////////////////////
//
//  File Name:          wrapper.rs
//
//  Purpose:            define a Rust-based front end for JSBSim
//
//  Components:
//  -wrapper_init()     instantiate and initialize a JSBSim Flight Dynamics Model
//  -send_to_jsbsim()   update FDM with data from the controller interface
//  -get_from_jsbsim()  iterate the flight dynamics model by one step
//                      provide property to allow scripts to end the simulation
//                      update the sensor interface with gyro data from jsbsim
//  -wrapper_close()    close the FDM and reset fdm to null (not implemented)
//  -extern block       provide rust-based front end for the c abi defined
//                          in wrapper.h & wrapper.cpp
//
///////////////////////////////////////////////////////////////////////////////

//meta
extern crate libc;
use std;
use std::ffi::CString;


///////////////////////////////////////////////////////////////////////////////
//
//  Function name:      wrapper_init
//
//  Purpose:            instantiate and initialize a JSBSim Flight Dynamics Model
//
//  Methodology:
//  -create a new fdm
//  -set base jsbsim scripts folder
//  -set jsbsim directory structure (aircraft, engine, systems) & verify
//  -load script & verify
//  -run initial conditions & verify
//                      
///////////////////////////////////////////////////////////////////////////////
pub fn wrapper_init(){

    //create a new instance of jsbsim
    unsafe{
        jsbsim = FDM::new() 
    };
    
    //get a reference to the new fdm
    let mut fdm = unsafe{&mut *jsbsim};    
    
    //set directory structure for jsbsim scripts
    let root_dir = CString::new("jsbsim/").unwrap();
    let aircraft_path = CString::new("aircraft").unwrap();
    let engine_path = CString::new("engine").unwrap();
    let systems_path = CString::new("systems").unwrap();
    fdm.set_dirs(root_dir, aircraft_path, engine_path, systems_path);
    
    //load script
    let script_name = std::ffi::CString::new("/scripts/testleds.xml").unwrap();
    let delta_t: f64 = 0.0;
    let init_file = std::ffi::CString::new("").unwrap();
    fdm.set_script(script_name, delta_t, init_file);
    
    //run initial conditions
    fdm.run_ic();
}


///////////////////////////////////////////////////////////////////////////////
//
//  Function name:      send_to_jsbsim
//
//  Purpose:            update FDM with data from the controller interface
//
//  Methodology:
//  -update fdm property testmode/ledcw with controller output cw
//  -update fdm property testmode/ledccw with controller output ccw
//  -this will indicate to the fdm that the flight controller is engaging
//  -the controller interface i.e. firing thrusters
//                        
///////////////////////////////////////////////////////////////////////////////
pub fn send_to_jsbsim(cw: u8, ccw: u8){

    //get a reference to the fdm
    let mut fdm = unsafe{&mut *jsbsim};
    
    //set cw value in jsbsim
    let property_cw = CString::new("testmode/ledcw").unwrap();
    fdm.set_property(property_cw, cw as f64);
    
    //set ccw value in jsbsim
    let property_ccw = CString::new("testmode/ledccw").unwrap();
    fdm.set_property(property_ccw, ccw as f64);
}


///////////////////////////////////////////////////////////////////////////////
//
//  Function name:      get_from_jsbsim
//
//  Purpose:            iterate the flight dynamics model by one step
//                      update the sensor interface with gyro data from jsbsim
//                      provide property to allow scripts to end the simulation
//
//  Methodology:
//  -iterate the flight dynamics model by one step and verify
//  -get sensor data
//  -check endscript property & exit application as indicated
//  -update the sensor interface with gyro data from jsbsim
//                        
///////////////////////////////////////////////////////////////////////////////
pub fn get_from_jsbsim()->(f32, f32, f32){
    
    //get a reference to the fdm
    let mut fdm = unsafe{&mut *jsbsim};

    let runresult: bool;
    
    //iterate jsbsim
    runresult = fdm.run();
    
    //if the script has completed, exit
    if runresult==false{
        panic!("temporary quit mechanism");
    }
        
    //get sensor data
    let gx: f64;
    let gy: f64 = 0.0;
    let gz: f64 = 0.0;
        
    let property_gyro = CString::new("testmode/gyro").unwrap();
    gx = fdm.get_property(property_gyro);
        
    //return gyro readings
    return (gx as f32, gy as f32, gz as f32);
}


///////////////////////////////////////////////////////////////////////////////
//
//  Function name:      wrapper_close
//
//  Purpose:            close the FDM and reset fdm to null (not implemented)
//
//  Methodology:
//  -close FDM
//  -set fdm to null
//                        
///////////////////////////////////////////////////////////////////////////////
pub fn wrapper_close(){
    //fdm_close(fdm);
    //fdm = 0 as *mut FDM;
}

///////////////////////////////////////////////////////////////////////////////
//
//  Component Name:     Rust Wrappers
//
//  Purpose:            These wrap the c-abi based rust functions definitions
//                          from the extern block (below) in more idiomatic
//                          Rust.  These are the functions that should be used
//                          in the wrapper functions (above).
//
//  function list:
//
///////////////////////////////////////////////////////////////////////////////
pub enum FDM{}
static mut jsbsim: *mut FDM = 0 as *mut FDM;

impl FDM{
    //creates a new instance of jsbsim & returns a pointer
    unsafe fn new()->*mut FDM{
        fdm_create()
    }
    
    //sets the directory structure
    fn set_dirs(&mut self, root_dir: CString, aircraft_path: CString, engine_path: CString, systems_path: CString){
        unsafe{
            //set base jsbsim scripts folder
            fdm_set_root_dir(self, root_dir.as_ptr());

            //set aircraft folder
            let apresult: bool;
            apresult = fdm_set_aircraft_path(self, aircraft_path.as_ptr());

            //set engine folder
            let epresult: bool;
            epresult = fdm_set_engine_path(self, engine_path.as_ptr());

            //set systems folder
            let spresult: bool;
            spresult = fdm_set_systems_path(self, systems_path.as_ptr());

            //verify directory structure was set properly
            if apresult == false || epresult == false || spresult == false {
            panic!("unable to configure directory structure");
            }
        }
    }
    
    //loads the script
    fn set_script(&mut self, script_name: CString, delta_t: f64, init_file: CString){
        unsafe{

            let lsresult: bool;

            //load script
            lsresult = fdm_load_script(self, script_name.as_ptr(), delta_t, init_file.as_ptr());
        
            //verify script load
            if lsresult == false{
                panic!("unable to load script");
            }
        }
    }
    
    //runs initial conditions
    fn run_ic(&mut self){
        //run initial conditions
        let icresult: bool;
        icresult = unsafe{
            fdm_run_ic(self)
        };
        
        //verify initial conditions
        if icresult == false{
            panic!("unable to set initial conditions");
        }
    }
    
    //gets the specified property
    fn get_property(&mut self, property_name: CString)->f64{
        unsafe{
            return fdm_get_property_double(self, property_name.as_ptr());
        }
    }
    
    //sets the specified property
    fn set_property(&mut self, property_name: CString, value: f64){
        unsafe{
            fdm_set_property_double(self, property_name.as_ptr(), value as f64);
        }
    }
    
    //iterates the fdm by one step
    fn run(&mut self)->bool{
        unsafe{
            return fdm_run(self);
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
//
//  Section name:       extern block
//
//  Purpose:            provide rust-based front end for the c abi defined
//                          in wrapper.h and wrapper.cpp
//
//  Methodology:
//  -provide linkage to JSBSim as a shared object or dynamic link library
//  -provide linkage to wrapper.h & wrapper.cpp as a static library
//  -provide a basic set of functions to access JSBSim via the c abi defined
//      in wrapper.h & wrapper.cpp
//
//  Notes:
//  -the functions in the extern block must parallel the c headers in wrapper.h
//                        
///////////////////////////////////////////////////////////////////////////////
#[link(name = "stdc++")]
#[link(name = "JSBSim")]
#[link(name = "wrapper", kind = "static")]

extern "C" {
    //jsbsim constructor
    fn fdm_create()->*mut FDM;

    //jsbsim destructor
    //fn fdm_close(fdm: *mut FDM);      //exit is not currently implemented

    //functions
    fn fdm_run(fdm: *mut FDM)->bool;
    fn fdm_run_ic(fdm: *mut FDM)->bool;
    fn fdm_load_script(fdm: *mut FDM, script_name: *const libc::c_char, delta_t: f64, init_file: *const libc::c_char)->bool;
    fn fdm_set_aircraft_path(fdm: *mut FDM, aircraft_path: *const libc::c_char)->bool;
    fn fdm_set_engine_path(fdm: *mut FDM, engine_path: *const libc::c_char)->bool;
    fn fdm_set_systems_path(fdm: *mut FDM, systems_path: *const libc::c_char)->bool;
    fn fdm_set_root_dir(fdm: *mut FDM, root_dir: *const libc::c_char);
    fn fdm_get_property_double(fdm: *mut FDM, property: *const libc::c_char)->f64;
    fn fdm_set_property_double(fdm: *mut FDM, property: *const libc::c_char, value: f64);
}
