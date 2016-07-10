#ifndef _JSBSIM_WRAPPER_H_
#define _JSBSIM_WRAPPER_H_

#include <FGFDMExec.h>
#include <iostream>

extern "C" {
	//rust api
	JSBSim::FGFDMExec* fdm_create();
	void fdm_get_version(JSBSim::FGFDMExec* ptr);
	void fdm_close(JSBSim::FGFDMExec* ptr);
}

#endif

