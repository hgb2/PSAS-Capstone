#include "JSBSimWrapper.h"

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
}

extern void wrapper_init()
{
	std::cout << "Initializing Wrapper" << std::endl;

	JSBSim::FGFDMExec* fdm = fdm_create();
	fdm_get_version(fdm);
	fdm_close(fdm);
}

extern void wrapper_update()
{

}

extern void wrapper_terminate()
{

}

