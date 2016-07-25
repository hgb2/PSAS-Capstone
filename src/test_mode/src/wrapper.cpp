//wrapper.cpp
//
//
#include "wrapper.h"

//test functions:  for development only
void wrapper_test(){
    std::cout << "wrapper_test" << std::endl;
    return;
}

//constructor
JSBSim::FGFDMExec* fdm_create(){
    return new JSBSim::FGFDMExec; 
}

//destructor
void fdm_close(JSBSim::FGFDMExec *fdm){
    //coordinate with both rust & jsbsim
    //delete fdm;
}

//functions 
//bool Run(void)
bool fdm_run(JSBSim::FGFDMExec *fdm){
    return fdm->Run();
}

//bool RunIC(void)
bool fdm_run_ic(JSBSim::FGFDMExec *fdm){
	return fdm->RunIC();
}

//bool LoadScript("run.xml")
bool fdm_load_script(JSBSim::FGFDMExec *fdm) {
	std::cerr << "Loading run.xml" << std::endl;
	bool success = fdm->LoadScript("run.xml");
	if(!success) {
		std::cerr << "Unable to load run.xml" << std::endl;
	}
	return success;
}

//FGInput* GetInput(void)
JSBSim::FGInput* fdm_get_input(JSBSim::FGFDMExec *fdm){
    return fdm->GetInput();
}

//FGOutput* GetOutput(void)
JSBSim::FGOutput* fdm_get_output(JSBSim::FGFDMExec *fdm){
    return fdm->GetOutput();
}

//from FGJSBBase class
std::string fdm_get_version(JSBSim::FGFDMExec *fdm){
	std::string version = fdm->GetVersion();
    
    //for testing:  remove in final
    std::cout << "wrapper version:  " << version << std::endl;
    return version;
}
