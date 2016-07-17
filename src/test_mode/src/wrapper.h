//testcode.h
#ifndef TESTCODE_H
#define TESTCODE_H

	//standard
	

	//jsbsim
	#include "FGFDMExec.h"
	#include <iostream>

	extern "C" {
		//jsbsim functions
		//constructor
		JSBSim::FGFDMExec* fdm_create();
		
		//function
		void fdm_get_version(JSBSim::FGFDMExec*);
		
		//deconstructor
		void fdm_close(JSBSim::FGFDMExec*);

		void wrapper_loopdata(JSBSim::FGFDMExec* fdm);
	}

#endif

