#ifndef _JSBSIM_WRAPPER_H_
#define _JSBSIM_WRAPPER_H_

#include <FGFDMExec.h>
#include <iostream>

extern "C" {
	//placeholder functions
	void wrapper_init();
	void wrapper_update();
	void wrapper_terminate();
	
	//rust api
	JSBSim::FGFDMExec* fdm_create();
	void fdm_get_version(JSBSim::FGFDMExec* fdm);
	void fdm_close(JSBSim::FGFDMExec* fdm);
}

#endif
