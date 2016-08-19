///////////////////////////////////////////////////////////////////////////////
//
//  File Name:          wrapper.h
//
//  Purpose:            provide a set of c function headers to wrap the c++
//                          calls used to access JSBSim
//
//  Components:
//  -constructor        return a pointer to a newly created Flight Dynamics
//                          Model
//  -destructor         (not implemented)
//  -functions          a set of basic function headers in c that are used by
//                          the wrapper.cpp file to wrap c++ function calls
//
///////////////////////////////////////////////////////////////////////////////
#ifndef WRAPPER_H
#define WRAPPER_H

//#includes
#include <iostream>
#include "FGFDMExec.h"
#include "math/FGLocation.h"
#include "input_output/FGGroundCallback.h"

//c abi
extern "C" {

    //constructor
    JSBSim::FGFDMExec* fdm_create();

    //deconstructor
    void fdm_close(JSBSim::FGFDMExec *fdm);

    //functions:
    bool fdm_run(JSBSim::FGFDMExec *fdm);
    bool fdm_run_ic(JSBSim::FGFDMExec *fdm);
    bool fdm_load_script(JSBSim::FGFDMExec *fdm, const char* script_name, double delta_t, const char* init_file);
    bool fdm_set_engine_path(JSBSim::FGFDMExec *fdm, const char* engine_path);
    bool fdm_set_aircraft_path(JSBSim::FGFDMExec *fdm, const char* aircraft_path);
    bool fdm_set_systems_path(JSBSim::FGFDMExec *fdm, const char* systems_path);
    void fdm_set_root_dir(JSBSim::FGFDMExec *fdm, const char* root_dir);
    double fdm_get_property_double(JSBSim::FGFDMExec *fdm, const char* property);
    void fdm_set_property_double(JSBSim::FGFDMExec *fdm, const char* property, double value);

}           //end extern block
#endif      //end #define WRAPPER_H
