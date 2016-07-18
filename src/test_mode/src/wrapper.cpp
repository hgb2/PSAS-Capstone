//wrapper.cpp
//
//
#include "wrapper.h"

//test functions:  for development only
void wrapper_test(){
    std::cout << "wrapper_test" << std::endl;
    return;
}
void wrapper_init(){
    std::cout << "wrapper_init" << std::endl;
    return;
}

void wrapper_step(){
    std::cout << "wrapper_loop" << std::endl;
    return;
}

void wrapper_close(){
    std::cout << "wrapper_init" << std::endl;
    return;
}

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

//void Unbind(void)
void fdm_unbind(JSBSim::FGFDMExec *fdm){
	fdm->Unbind();
}

//bool Run(void)
bool fdm_run(JSBSim::FGFDMExec *fdm){
    return fdm->Run();
}

//bool RunIC(void)
bool fdm_run_ic(JSBSim::FGFDMExec *fdm){
	return fdm->RunIC();
}

//pointer
//void SetGroundCallback(FGGroundCallback* gc)
void fdm_set_ground_callback(JSBSim::FGFDMExec *fdm, JSBSim::FGGroundCallback *gc){
    fdm->SetGroundCallback(gc);
}

//add_model_to_path initialized in implementation
//bool LoadModel(const std::string& AircraftPath, const std::string& EnginePath, const std::string& SystemsPath, const std::string& model, bool addModelToPath = true);
bool fdm_load_model_ext(JSBSim::FGFDMExec *fdm, std::string aircraft_path, std::string engine_path, std::string systems_path, std::string model, bool add_model_to_path){
    add_model_to_path = true;
    return fdm->LoadModel(aircraft_path, engine_path, systems_path, model, add_model_to_path);
}

//add_model_to_path initialized in implementation
//bool LoadModel(const std::string& model, bool addModelToPath = true);
bool fdm_load_model(JSBSim::FGFDMExec *fdm, std::string model, bool add_model_to_path){
    add_model_to_path = true;
    return fdm->LoadModel(model, add_model_to_path);
}

//bool LoadScript(const std::string& Script, double deltaT=0.0, const std::string&                  initfile="");
bool fdm_load_script(JSBSim::FGFDMExec *fdm, std::string script_name, double delta_t, std::string init_file){
    return fdm->LoadScript(script_name, delta_t, init_file);
}

//bool SetEnginePath(const std::string& path)
bool fdm_set_engine_path(JSBSim::FGFDMExec *fdm, std::string engine_path){
	return fdm->SetEnginePath(engine_path);
}

//bool SetAircraftPath(const std::string& path)
bool fdm_set_aircraft_path(JSBSim::FGFDMExec *fdm, std::string aircraft_path){
	return fdm->SetAircraftPath(aircraft_path);
}

//bool SetSystemsPath(const std::string& path)
bool fdm_set_systems_path(JSBSim::FGFDMExec *fdm, std::string systems_path){
	return fdm->SetSystemsPath(systems_path);
}

//top level executive state and model retrieval mechanism
//FGAtmosphere* GetAtmosphere(void)
JSBSim::FGAtmosphere* fdm_get_atmosphere(JSBSim::FGFDMExec *fdm){
    return fdm->GetAtmosphere();
}

//FGAccelerations* GetAccelerations(void)
JSBSim::FGAccelerations* fdm_get_accelerations(JSBSim::FGFDMExec *fdm){
    return fdm->GetAccelerations();
}

//FGWinds* GetWinds(void)
JSBSim::FGWinds* fdm_get_winds(JSBSim::FGFDMExec *fdm){
    return fdm->GetWinds();
}

//FGFCS* GetFCS(void)
JSBSim::FGFCS* fdm_get_fcs(JSBSim::FGFDMExec *fdm){
    return fdm->GetFCS();
}

//FGPropulsion* GetPropulsion(void)
JSBSim::FGPropulsion* fdm_get_propulsion(JSBSim::FGFDMExec *fdm){
    return fdm->GetPropulsion();
}

//FGMassBalance* GetMassBalance(void)
JSBSim::FGMassBalance* fdm_get_mass_balance(JSBSim::FGFDMExec *fdm){
    return fdm->GetMassBalance();
}

//FGAerodynamics* GetAerodynamics(void)
JSBSim::FGAerodynamics* fdm_get_aerodynamics(JSBSim::FGFDMExec *fdm){
    return fdm->GetAerodynamics();
}

//FGInertial* GetInertial(void)
JSBSim::FGInertial* fdm_get_inertial(JSBSim::FGFDMExec *fdm){
    return fdm->GetInertial();
}

//FGGroundReactions* GetGroundReactions(void)
JSBSim::FGGroundReactions* fdm_get_ground_reactions(JSBSim::FGFDMExec *fdm){
    return fdm->GetGroundReactions();
}

//FGExternalReactions* GetExternalReactions(void)
JSBSim::FGExternalReactions* fdm_get_external_reactions(JSBSim::FGFDMExec *fdm){
    return fdm->GetExternalReactions();
}

//FGBuoyantForces* GetBuoyantForces(void)
JSBSim::FGBuoyantForces* fdm_get_buoyant_forces(JSBSim::FGFDMExec *fdm){
    return fdm->GetBuoyantForces();
}

//FGAircraft* GetAircraft(void)
JSBSim::FGAircraft* fdm_get_aircraft(JSBSim::FGFDMExec *fdm){
    return fdm->GetAircraft();
}

//FGPropagate* GetPropagate(void)
JSBSim::FGPropagate* fdm_get_propagate(JSBSim::FGFDMExec *fdm){
    return fdm->GetPropagate();
}

//FGAuxiliary* GetAuxiliary(void)
JSBSim::FGAuxiliary* fdm_get_auxiliary(JSBSim::FGFDMExec *fdm){
    return fdm->GetAuxiliary();
}

//FGInput* GetInput(void)
JSBSim::FGInput* fdm_get_input(JSBSim::FGFDMExec *fdm){
    return fdm->GetInput();
}

//FGOutput* GetOutput(void)
JSBSim::FGOutput* fdm_get_output(JSBSim::FGFDMExec *fdm){
    return fdm->GetOutput();
}

//FGGroundCallback* GetGroundCallback(void)
JSBSim::FGGroundCallback* fdm_get_ground_callback(JSBSim::FGFDMExec *fdm){
    return fdm->GetGroundCallback();
}

//FGScript* GetScript(void)
JSBSim::FGScript* fdm_get_script(JSBSim::FGFDMExec *fdm){
    return fdm->GetScript();
}

//FGInitialCondition* GetIC(void)
JSBSim::FGInitialCondition* fdm_get_initial_condition(JSBSim::FGFDMExec *fdm){
    return fdm->GetIC();
}

//FGTrim* GetTrim(void);
JSBSim::FGTrim* fdm_get_trim(JSBSim::FGFDMExec *fdm){
    return fdm->GetTrim();
}

//const std::string& GetEnginePath(void)
std::string fdm_get_engine_path(JSBSim::FGFDMExec *fdm){
    return fdm->GetEnginePath();
}
        
//const std::string& GetAircraftPath(void)
std::string fdm_get_aircraft_path(JSBSim::FGFDMExec *fdm){
    return fdm->GetAircraftPath();
}
        
//const std::string& GetSystemsPath(void)
std::string fdm_get_systems_path(JSBSim::FGFDMExec *fdm){
    return fdm->GetSystemsPath();
}

//const std::string& GetFullAircraftPath(void)
std::string fdm_get_full_aircraft_path(JSBSim::FGFDMExec *fdm){
    return fdm->GetFullAircraftPath();
}

//inline double GetPropertyValue(const std::string& property)
double fdm_get_property_value(JSBSim::FGFDMExec *fdm, std::string property){
    return fdm->GetPropertyValue(property);
}

//inline void SetPropertyValue(const std::string& property, double value)
void fdm_set_property_value(JSBSim::FGFDMExec *fdm, std::string property, double value){
    fdm->SetPropertyValue(property, value);
}

//const std::string& GetModelName(void)
std::string fdm_get_model_name(JSBSim::FGFDMExec *fdm){
    return fdm->GetModelName();
}

//FGPropertyManager* GetPropertyManager(void);
JSBSim::FGPropertyManager* fdm_get_property_manager(JSBSim::FGFDMExec *fdm){
    return fdm->GetPropertyManager();
}

//vectors
//std::vector <std::string> EnumerateFDMs(void);

//int GetFDMCount(void) const {return (int)ChildFDMList.size();}
int fdm_get_fdm_count(JSBSim::FGFDMExec *fdm){
    return fdm->GetFDMCount();
}

//struct
//childData* GetChildFDM(int i) const {return ChildFDMList[i];}
//FGFDMExec::childData* fdm_get_child_data(JSBSim::FGFDMExec *fdm, int i){
//    JSBSim::FGFDMExec::childData *childData;
//    childData = fdm->GetChildFDM(i);
//    return childData;
//}

//void SetChild(bool ch)
void fdm_set_child(JSBSim::FGFDMExec *fdm, bool ch){
    fdm->SetChild(ch);
}

//const
//bool SetOutputDirectives(const std::string& fname)
bool fdm_set_output_directives(JSBSim::FGFDMExec *fdm, std::string fname){
    return fdm->SetOutputDirectives(fname);
}

//weird initialization
//void ForceOutput(int idx=0)
void fdm_force_output(JSBSim::FGFDMExec *fdm, int idx){
    idx = 0;
    fdm->ForceOutput(idx);
}

//void SetLoggingRate(double rate)
void fdm_set_logging_rate(JSBSim::FGFDMExec *fdm, double rate){
    fdm->SetLoggingRate(rate);
}

//check const
//bool SetOutputFileName(const int n, const std::string& fname)
bool fdm_set_output_filename(JSBSim::FGFDMExec *fdm, int n, std::string fname){
    return fdm->SetOutputFileName(n, fname);
}


//std::string GetOutputFileName(int n)
std::string fdm_get_output_filename(JSBSim::FGFDMExec *fdm, int n){
    return fdm->GetOutputFileName(n);
}

//void DoTrim(int mode);
void fdm_do_trim(JSBSim::FGFDMExec *fdm, int mode){
    fdm->DoTrim(mode);
}

//void DisableOutput(void)
void fdm_disable_output(JSBSim::FGFDMExec *fdm){
    fdm->DisableOutput();
}

//void EnableOutput(void)
void fdm_enable_output(JSBSim::FGFDMExec *fdm){
    fdm->EnableOutput();
}

//void Hold(void)
void fdm_hold(JSBSim::FGFDMExec *fdm){
    fdm->Hold();
}

//void EnableIncrementThenHold(int Timesteps)
void fdm_enable_increment_then_hold(JSBSim::FGFDMExec *fdm, int timesteps){
    fdm->EnableIncrementThenHold(timesteps);
}

//void CheckIncrementalHold(void);
void fdm_check_incremental_hold(JSBSim::FGFDMExec *fdm){
    fdm->CheckIncrementalHold();
}

//void Resume(void)
void fdm_resume(JSBSim::FGFDMExec *fdm){
    fdm->Resume();
}

//bool Holding(void)
bool fdm_holding(JSBSim::FGFDMExec *fdm){
    return fdm->Holding();
}


//void ResetToInitialConditions(int mode);
void fdm_reset_to_initial_conditions(JSBSim::FGFDMExec *fdm, int mode){
    fdm->ResetToInitialConditions(mode);
}


//void SetDebugLevel(int level)
void fdm_set_debug_level(JSBSim::FGFDMExec *fdm, int level){
    fdm->SetDebugLevel(level);
}


//void BuildPropertyCatalog(struct PropertyCatalogStructure* pcs);
void fdm_build_property_catalog(JSBSim::FGFDMExec *fdm, JSBSim::FGFDMExec::PropertyCatalogStructure *pcs);

//std::string QueryPropertyCatalog(const std::string& check);
std::string fdm_query_property_catalog(JSBSim::FGFDMExec *fdm, std::string check){
    return fdm->QueryPropertyCatalog(check);
}

//void PrintPropertyCatalog(void)
void fdm_print_property_catalog(JSBSim::FGFDMExec *fdm){
    fdm->PrintPropertyCatalog();
}

//void PrintSimulationConfiguration(void) const
void fdm_print_simulation_configuration(JSBSim::FGFDMExec *fdm){
    fdm->PrintSimulationConfiguration();
}

//vectors
//std::vector<std::string>& GetPropertyCatalog(void) {return PropertyCatalog;}
//std::vector<std::string> fdm_get_property_catalog(JSBSim::FGFDMExec *fdm){
//  std::vector<std::string> property_catalog;
//  property_catalog = fdm->GetPropertyCatalog();
//  return property_catalog;
//}

//void SetTrimStatus(bool status)
void fdm_set_trim_status(JSBSim::FGFDMExec *fdm, bool status){
    fdm->SetTrimStatus(status);
}
        
//bool GetTrimStatus(void)
bool fdm_get_trim_status(JSBSim::FGFDMExec *fdm){
    return fdm->GetTrimStatus();
}
        
//void SetTrimMode(int mode)
void fdm_set_trim_mode(JSBSim::FGFDMExec *fdm, int mode){
    fdm->SetTrimMode(mode);
}
        
//int GetTrimMode(void)
int fdm_get_trim_mode(JSBSim::FGFDMExec *fdm){
    return fdm->GetTrimMode();
}

//std::string GetPropulsionTankReport();
std::string fdm_get_propulsion_tank_report(JSBSim::FGFDMExec *fdm){
    return fdm->GetPropulsionTankReport();
}

//double GetSimTime(void)
double fdm_get_sim_time(JSBSim::FGFDMExec *fdm){
    return fdm->GetSimTime();
}

//double GetDeltaT(void)
double fdm_get_delta_t(JSBSim::FGFDMExec *fdm){
    return fdm->GetDeltaT();
}

//void SuspendIntegration(void)
void fdm_suspend_integration(JSBSim::FGFDMExec *fdm){
    fdm->SuspendIntegration();
}

//void ResumeIntegration(void)
void fdm_resume_integration(JSBSim::FGFDMExec *fdm){
    fdm->ResumeIntegration();
}

//bool IntegrationSuspended(void)
bool fdm_integration_suspended(JSBSim::FGFDMExec *fdm){
    return fdm->IntegrationSuspended();
}

//double Setsim_time(double cur_time)
double fdm_set_sim_time(JSBSim::FGFDMExec *fdm, double cur_time){
    return fdm->Setsim_time(cur_time);
}

//void Setdt(double delta_t)
void fdm_set_dt(JSBSim::FGFDMExec *fdm, double delta_t){
    fdm->Setdt(delta_t);
}

//void SetRootDir(const std::string& rootDir)
void fdm_set_root_dir(JSBSim::FGFDMExec *fdm, std::string root_dir)
{
	fdm->SetRootDir(root_dir);
}

//const std::string& GetRootDir(void)
std::string fdm_get_root_dir(JSBSim::FGFDMExec *fdm){
    return fdm->GetRootDir();
}

//double IncrTime(void)
double fdm_incr_time(JSBSim::FGFDMExec *fdm){
    return fdm->IncrTime();
}

//unsigned int GetFrame(void)
unsigned int fdm_get_frame(JSBSim::FGFDMExec *fdm){
    return fdm->GetFrame();
}

//int GetDebugLevel(void)
int fdm_get_debug_level(JSBSim::FGFDMExec *fdm){
    return fdm->GetDebugLevel();
}

//void Initialize(FGInitialCondition *FGIC);
void fdm_initialize(JSBSim::FGFDMExec *fdm, JSBSim::FGInitialCondition *FGIC){
    fdm->Initialize(FGIC);
}

//void SetHoldDown(bool hd);
void fdm_set_hold_down(JSBSim::FGFDMExec *fdm, bool hold_down){
    fdm->SetHoldDown(hold_down);
}

//bool GetHoldDown(void)
bool fdm_get_hold_down(JSBSim::FGFDMExec *fdm){
    return fdm->GetHoldDown();
}

//from FGJSBBase class
std::string fdm_get_version(JSBSim::FGFDMExec *fdm){
	std::string version = fdm->GetVersion();
    
    //for testing:  remove in final
    std::cout << "wrapper version:  " << version << std::endl;
    return version;
}
