#include "JSBSimWrapper.h"

//jsbsim constructor wrapper
extern JSBSim::FGFDMExec* fdm_create()
{
	JSBSim::FGFDMExec* fdm = new JSBSim::FGFDMExec();
	std::cout << "JSBSim Initialized." << std::endl;
	return fdm;
}

//jsbsim function wrapper
extern void fdm_get_version(JSBSim::FGFDMExec* fdm)
{
	string version = fdm->GetVersion();
	std::cout << "JSBSim Version:  " << version << "." << std::endl;
}

//jsbsim deconstructor wrapper
extern void fdm_close(JSBSim::FGFDMExec* fdm)
{
	//close out jsbsim
	
	
	//test message:
	std::cout << "JSBSim Closed.\n" << std::endl;
}

