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
		
//wrapper test functions (development only)
void wrapper_test();

//JSBSim constructor
JSBSim::FGFDMExec* fdm_create();
		
//JSBSim deconstructor
void fdm_close(JSBSim::FGFDMExec *fdm);		
		
//JSBSim functions: 
//the original function definition from JSBSim::FGFDMExec is listed for reference

//bool Run(void);
bool fdm_run(JSBSim::FGFDMExec *fdm);
        
//bool RunIC(void);     
bool fdm_run_ic(JSBSim::FGFDMExec *fdm);

//bool LoadScript(const std::string& Script, double deltaT=0.0, const std::string& initfile="");
bool fdm_load_script(JSBSim::FGFDMExec *fdm, std::string script_name, double delta_t, std::string init_file);

//FGInput* GetInput(void)
JSBSim::FGInput* fdm_get_input(JSBSim::FGFDMExec *fdm);

//FGOutput* GetOutput(void)
JSBSim::FGOutput* fdm_get_output(JSBSim::FGFDMExec *fdm);

//from FGJSBBase class
//string GetVersion()
std::string fdm_get_version(JSBSim::FGFDMExec *fdm);

}           //end extern block
#endif      //end #define WRAPPER_H
