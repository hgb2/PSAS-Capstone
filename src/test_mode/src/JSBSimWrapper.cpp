#include "JSBSimWrapper.h"

void wrapper_init()
{
	std::cout << "Initializing Wrapper" << std::endl;
}

//jsbsim constructor wrapper
JSBSim::FGFDMExec* fdm_create()
{
	JSBSim::FGFDMExec * fdm = new JSBSim::FGFDMExec();
	std::cout << "JSBSim Initialized." << std::endl;
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
	std::cout << "Daylight come and we want to go home." << std::endl;
}


void wrapper_update()
{

}

void wrapper_terminate()
{

}

