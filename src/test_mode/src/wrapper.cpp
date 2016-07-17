//wrapper.cpp
//
//

//project #includes
#include "wrapper.h"

//jsbsim constructor wrapper
JSBSim::FGFDMExec* fdm_create()
{
	JSBSim::FGFDMExec* fdm = new JSBSim::FGFDMExec();
	std::cout << "JSBSim Initialized." << std::endl;

	//fdm->LoadScript("run.xml");
	return fdm;
}

//jsbsim function wrapper
void fdm_get_version(JSBSim::FGFDMExec* fdm)
{
	string version = fdm->GetVersion();
	std::cout << "JSBSim Version:  " << version << "." << std::endl;
}

//jsbsim deconstructor wrapper
void fdm_close(JSBSim::FGFDMExec* fdm)
{
	delete fdm;
	std::cout << "JSBSim Closed.\n" << std::endl;
}

void wrapper_loopdata(JSBSim::FGFDMExec* fdm)
{
	// Step the JSBSim simulation
	fdm->Run();
}

