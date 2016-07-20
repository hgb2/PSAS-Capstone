//wrapper.cpp
//
//
#include "wrapper.h"

//constructor
JSBSim::FGFDMExec* fdm_create(){
    return new JSBSim::FGFDMExec; 
}

//destructor
void fdm_close(JSBSim::FGFDMExec *fdm){
	//coordinate with both rust & jsbsim
    //delete fdm;
}

//temp function
void fdm_get_version(JSBSim::FGFDMExec *fdm){
	std::string version = fdm->GetVersion();
    
    //for testing:  remove in final
    std::cout << "wrapper: version:  " << version << std::endl;
}
