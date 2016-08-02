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

//functions 
bool fdm_run(JSBSim::FGFDMExec *fdm){
    return fdm->Run();
}

bool fdm_run_ic(JSBSim::FGFDMExec *fdm){
	return fdm->RunIC();
}

bool fdm_load_script(JSBSim::FGFDMExec *fdm, const char* script_name, double delta_t, const char* init_file){
    std::string script_name_cpp = script_name;
    std::string init_file_cpp = init_file;
    return fdm->LoadScript(script_name_cpp, delta_t, init_file_cpp="");
}

bool fdm_set_engine_path(JSBSim::FGFDMExec *fdm, const char* engine_path){
    std::string engine_path_cpp = engine_path;
	return fdm->SetEnginePath(engine_path_cpp);
}

bool fdm_set_aircraft_path(JSBSim::FGFDMExec *fdm, const char* aircraft_path){
    std::string aircraft_path_cpp = aircraft_path;
	return fdm->SetAircraftPath(aircraft_path_cpp);
}

bool fdm_set_systems_path(JSBSim::FGFDMExec *fdm, const char* systems_path){
    std::string systems_path_cpp = systems_path;
	return fdm->SetSystemsPath(systems_path_cpp);
}

void fdm_set_root_dir(JSBSim::FGFDMExec *fdm, const char* root_dir){
    std::string root_dir_cpp = root_dir;
	fdm->SetRootDir(root_dir_cpp);
}

