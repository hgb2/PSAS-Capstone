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


extern crate libc;
use std;

//type: FGFDMExec
pub enum FDM{}
static mut fdm: *mut FDM = 0 as *mut FDM;


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

        //verify directory structure was set properly
        if apresult == false || epresult == false || spresult == false {
            panic!("unable to configure directory structure");
        }
        
        //load script prep
        let script_name = std::ffi::CString::new("/scripts/testleds.xml").unwrap();
        let delta_t: f64 = 0.0;
        let init_file = std::ffi::CString::new("").unwrap();
        let lsresult: bool;

        //load script
        lsresult = fdm_load_script(fdm, script_name.as_ptr(), delta_t, init_file.as_ptr());
        
        //verify script load
        if lsresult == false{
            panic!("unable to load script");
        }

        //run initial conditions
        let icresult: bool;
        icresult = fdm_run_ic(fdm);
        
        //verify initial conditions
        if icresult == false{
            panic!("unable to set initial conditions");
        }
    }
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

    unsafe{
<<<<<<< HEAD
=======
       
>>>>>>> 20fd50b5582941dfa9ccb8032026341e806e3eaa
        //set cw & ccw values in jsbsim
        let property_cw = std::ffi::CString::new("testmode/ledcw").unwrap();
        fdm_set_property_double(fdm, property_cw.as_ptr(), cw as f64);
        
        let property_ccw = std::ffi::CString::new("testmode/ledccw").unwrap();
        fdm_set_property_double(fdm, property_ccw.as_ptr(), ccw as f64);
    }
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

    let runresult: bool;
    
    unsafe{
        //iterate jsbsim & verify
        runresult = fdm_run(fdm);
        if runresult==false{
            panic!("jsbsim did not iterate properly");
        }
        
        //get sensor data
        let gx: f64;
        let gy: f64 = 0.0;
        let gz: f64 = 0.0;
        
        let property_gyro = std::ffi::CString::new("testmode/gyro").unwrap();
        gx = fdm_get_property_double(fdm, property_gyro.as_ptr());
        
        //temporary quit mechanism
        let endscript: f64;
        let property_endscript = std::ffi::CString::new("testmode/endscript").unwrap();
        endscript = fdm_get_property_double(fdm, property_endscript.as_ptr());
        
        
        if endscript > 0.0{
            println!("endscript:\t{}", endscript);
            panic!("temporary quit mechanism");
        }
                
        //return gyro readings
        return (gx as f32, gy as f32, gz as f32);
    }
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
