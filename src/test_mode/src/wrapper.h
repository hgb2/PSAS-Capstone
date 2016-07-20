//wrapper.h
//
//
#ifndef WRAPPER_H
#define WRAPPER_H

//#includes
#include <iostream>
	
#include "FGFDMExec.h"
#include "math/FGLocation.h"
#include "input_output/FGGroundCallback.h"

//c api
extern "C" {
		
//JSBSim constructor
JSBSim::FGFDMExec* fdm_create();
		
//JSBSim deconstructor
void fdm_close(JSBSim::FGFDMExec *fdm);		
		
//JSBSim functions: 
//the original function definition from JSBSim::FGFDMExec is listed for reference

//test function
//note that the original returns std::string
//string GetVersion()
void fdm_get_version(JSBSim::FGFDMExec *fdm);

}           //end extern block
#endif      //end #define WRAPPER_H
