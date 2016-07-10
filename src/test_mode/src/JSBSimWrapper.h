#ifndef _JSBSIM_WRAPPER_H_
#define _JSBSIM_WRAPPER_H_

#include <FGFDMExec.h>
#include <iostream>

extern "C" {
	void wrapper_init();
	void wrapper_update();
	void wrapper_terminate();
}

#endif
