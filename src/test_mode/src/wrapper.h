//wrapper.h
//
//
#ifndef WRAPPER_H
#define WRAPPER_H

//#includes
#include <iostream>
	
#include "FGFDMExec.h"
#include "math/FGLocation.h"
#include "input_output/FGGroundCallback.h"

//c api
extern "C" {
		
//wrapper test functions (development only)
void wrapper_test();
void wrapper_init();
void wrapper_step();
void wrapper_close();
		
//JSBSim constructor
JSBSim::FGFDMExec* fdm_create();
		
//JSBSim deconstructor
void fdm_close(JSBSim::FGFDMExec *fdm);		
		
//JSBSim functions: 
//the original function definition from JSBSim::FGFDMExec is listed for reference

//void Unbind(void)
void fdm_unbind(JSBSim::FGFDMExec *fdm);
        
//bool Run(void);
bool fdm_run(JSBSim::FGFDMExec *fdm);
        
//bool RunIC(void);     
bool fdm_run_ic(JSBSim::FGFDMExec *fdm);

//pointer
//void SetGroundCallback(FGGroundCallback* gc)
void fdm_set_ground_callback(JSBSim::FGFDMExec *fdm, JSBSim::FGGroundCallback *gc);

//add_model_to_path initialized in implementation
//bool LoadModel(const std::string& AircraftPath, const std::string& EnginePath, const std::string& SystemsPath, const std::string& model, bool addModelToPath = true);
bool fdm_load_model_ext(JSBSim::FGFDMExec *fdm, std::string aircraft_path, std::string engine_path, std::string systems_path, std::string model, bool add_model_to_path);

//add_model_to_path initialized in implementation
//bool LoadModel(const std::string& model, bool addModelToPath = true);
bool fdm_load_model(JSBSim::FGFDMExec *fdm, std::string model, bool add_model_to_path);

//bool LoadScript(const std::string& Script, double deltaT=0.0, const std::string& initfile="");
bool fdm_load_script(JSBSim::FGFDMExec *fdm, std::string script_name, double delta_t, std::string init_file);
        
//bool SetEnginePath(const std::string& path)
bool fdm_set_engine_path(JSBSim::FGFDMExec *fdm, std::string engine_path);
		
//bool SetAircraftPath(const std::string& path)
bool fdm_set_aircraft_path(JSBSim::FGFDMExec *fdm, std::string aircraft_path);
		
//bool SetSystemsPath(const std::string& path)
bool fdm_set_systems_path(JSBSim::FGFDMExec *fdm, std::string systems_path);
				
//top level executive state and model retrieval mechanism
//FGAtmosphere* GetAtmosphere(void)
JSBSim::FGAtmosphere* fdm_get_atmosphere(JSBSim::FGFDMExec *fdm);

//FGAccelerations* GetAccelerations(void)
JSBSim::FGAccelerations* fdm_get_accelerations(JSBSim::FGFDMExec *fdm);

//FGWinds* GetWinds(void)
JSBSim::FGWinds* fdm_get_winds(JSBSim::FGFDMExec *fdm);

//FGFCS* GetFCS(void)
JSBSim::FGFCS* fdm_get_fcs(JSBSim::FGFDMExec *fdm);

//FGPropulsion* GetPropulsion(void)
JSBSim::FGPropulsion* fdm_get_propulsion(JSBSim::FGFDMExec *fdm);

//FGMassBalance* GetMassBalance(void)
JSBSim::FGMassBalance* fdm_get_mass_balance(JSBSim::FGFDMExec *fdm);

//FGAerodynamics* GetAerodynamics(void)
JSBSim::FGAerodynamics* fdm_get_aerodynamics(JSBSim::FGFDMExec *fdm);

//FGInertial* GetInertial(void)
JSBSim::FGInertial* fdm_get_inertial(JSBSim::FGFDMExec *fdm);

//FGGroundReactions* GetGroundReactions(void)
JSBSim::FGGroundReactions* fdm_get_ground_reactions(JSBSim::FGFDMExec *fdm);

//FGExternalReactions* GetExternalReactions(void)
JSBSim::FGExternalReactions* fdm_get_external_reactions(JSBSim::FGFDMExec *fdm);

//FGBuoyantForces* GetBuoyantForces(void)
JSBSim::FGBuoyantForces* fdm_get_buoyant_forces(JSBSim::FGFDMExec *fdm);

//FGAircraft* GetAircraft(void)
JSBSim::FGAircraft* fdm_get_aircraft(JSBSim::FGFDMExec *fdm);

//FGPropagate* GetPropagate(void)
JSBSim::FGPropagate* fdm_get_propagate(JSBSim::FGFDMExec *fdm);

//FGAuxiliary* GetAuxiliary(void)
JSBSim::FGAuxiliary* fdm_get_auxiliary(JSBSim::FGFDMExec *fdm);

//FGInput* GetInput(void)
JSBSim::FGInput* fdm_get_input(JSBSim::FGFDMExec *fdm);

//FGOutput* GetOutput(void)
JSBSim::FGOutput* fdm_get_output(JSBSim::FGFDMExec *fdm);

//FGGroundCallback* GetGroundCallback(void)
JSBSim::FGGroundCallback* fdm_get_ground_callback(JSBSim::FGFDMExec *fdm);

//FGScript* GetScript(void)
JSBSim::FGScript* fdm_get_script(JSBSim::FGFDMExec *fdm);

//FGInitialCondition* GetIC(void)
JSBSim::FGInitialCondition* fdm_get_initial_condition(JSBSim::FGFDMExec *fdm);

//FGTrim* GetTrim(void);
JSBSim::FGTrim* fdm_get_trim(JSBSim::FGFDMExec *fdm);

//const std::string& GetEnginePath(void)
std::string fdm_get_engine_path(JSBSim::FGFDMExec *fdm);
        
//const std::string& GetAircraftPath(void)
std::string fdm_get_aircraft_path(JSBSim::FGFDMExec *fdm);
        
//const std::string& GetSystemsPath(void)
std::string fdm_get_systems_path(JSBSim::FGFDMExec *fdm);

//const std::string& GetFullAircraftPath(void)
std::string fdm_get_full_aircraft_path(JSBSim::FGFDMExec *fdm);

//inline double GetPropertyValue(const std::string& property)
double fdm_get_property_value(JSBSim::FGFDMExec *fdm, std::string property);

//inline void SetPropertyValue(const std::string& property, double value)
void fdm_set_property_value(JSBSim::FGFDMExec *fdm, std::string property, double value);

//const std::string& GetModelName(void)
std::string fdm_get_model_name(JSBSim::FGFDMExec *fdm);

//FGPropertyManager* GetPropertyManager(void)
JSBSim::FGPropertyManager* fdm_get_property_manager(JSBSim::FGFDMExec *fdm);

//vectors
//std::vector <std::string> EnumerateFDMs(void)

//int GetFDMCount(void)
int fdm_get_fdm_count(JSBSim::FGFDMExec *fdm);

//void SetChild(bool ch)
void fdm_set_child(JSBSim::FGFDMExec *fdm, bool ch);

//const
//bool SetOutputDirectives(const std::string& fname)
bool fdm_set_output_directives(JSBSim::FGFDMExec *fdm, std::string fname);

//idx set to 0 in implementation
//void ForceOutput(int idx=0) 
void fdm_force_output(JSBSim::FGFDMExec *fdm, int idx);

//void SetLoggingRate(double rate)
void fdm_set_logging_rate(JSBSim::FGFDMExec *fdm, double rate);

//check const
//bool SetOutputFileName(const int n, const std::string& fname)
bool fdm_set_output_filename(JSBSim::FGFDMExec *fdm, int n, std::string fname);

//std::string GetOutputFileName(int n)
std::string fdm_get_output_filename(JSBSim::FGFDMExec *fdm, int n);

//void DoTrim(int mode);
void fdm_do_trim(JSBSim::FGFDMExec *fdm, int mode);

//void DisableOutput(void)
void fdm_disable_output(JSBSim::FGFDMExec *fdm);

//void EnableOutput(void)
void fdm_enable_output(JSBSim::FGFDMExec *fdm);

//void Hold(void)
void fdm_hold(JSBSim::FGFDMExec *fdm);

//void EnableIncrementThenHold(int Timesteps)
void fdm_enable_increment_then_hold(JSBSim::FGFDMExec *fdm, int timesteps);

//void CheckIncrementalHold(void);
void fdm_check_incremental_hold(JSBSim::FGFDMExec *fdm);

//void Resume(void)
void fdm_resume(JSBSim::FGFDMExec *fdm);

//bool Holding(void)
bool fdm_holding(JSBSim::FGFDMExec *fdm);

//void ResetToInitialConditions(int mode);
void fdm_reset_to_initial_conditions(JSBSim::FGFDMExec *fdm, int mode);

//void SetDebugLevel(int level)
void fdm_set_debug_level(JSBSim::FGFDMExec *fdm, int level);

//struct PropertyCatalogStructure
//does this need to be defined in the wrapper?

//void BuildPropertyCatalog(struct PropertyCatalogStructure* pcs);
void fdm_build_property_catalog(JSBSim::FGFDMExec *fdm, JSBSim::FGFDMExec::PropertyCatalogStructure *pcs);

//std::string QueryPropertyCatalog(const std::string& check);
std::string fdm_query_property_catalog(JSBSim::FGFDMExec *fdm, std::string check);
        
//void PrintPropertyCatalog(void)
void fdm_print_property_catalog(JSBSim::FGFDMExec *fdm);
        
//void PrintSimulationConfiguration(void)
void fdm_print_simulation_configuration(JSBSim::FGFDMExec *fdm);

//vectors
//std::vector<std::string>& GetPropertyCatalog(void)
        
//void SetTrimStatus(bool status)
void fdm_set_trim_status(JSBSim::FGFDMExec *fdm, bool status);
        
//bool GetTrimStatus(void)
bool fdm_get_trim_status(JSBSim::FGFDMExec *fdm);
        
//void SetTrimMode(int mode)
void fdm_set_trim_mode(JSBSim::FGFDMExec *fdm, int mode);
        
//int GetTrimMode(void)
int fdm_get_trim_mode(JSBSim::FGFDMExec *fdm);

//std::string GetPropulsionTankReport();
std::string fdm_get_propulsion_tank_report(JSBSim::FGFDMExec *fdm);
        
//double GetSimTime(void)
double fdm_get_sim_time(JSBSim::FGFDMExec *fdm);
        
//double GetDeltaT(void)
double fdm_get_delta_t(JSBSim::FGFDMExec *fdm);
        
//void SuspendIntegration(void)
void fdm_suspend_integration(JSBSim::FGFDMExec *fdm);        
        
//void ResumeIntegration(void)
void fdm_resume_integration(JSBSim::FGFDMExec *fdm);
                
//bool IntegrationSuspended(void)
bool fdm_integration_suspended(JSBSim::FGFDMExec *fdm);
        
//double Setsim_time(double cur_time)
double fdm_set_sim_time(JSBSim::FGFDMExec *fdm, double cur_time);
        
//void Setdt(double delta_t)
void fdm_set_dt(JSBSim::FGFDMExec *fdm, double delta_t);
        
//void SetRootDir(const std::string& rootDir)
void fdm_set_root_dir(JSBSim::FGFDMExec *fdm, std::string root_dir);

//const std::string& GetRootDir(void)
std::string fdm_get_root_dir(JSBSim::FGFDMExec *fdm);

//double IncrTime(void)
double fdm_incr_time(JSBSim::FGFDMExec *fdm);
                
//unsigned int GetFrame(void)
unsigned int fdm_get_frame(JSBSim::FGFDMExec *fdm);
        
//int GetDebugLevel(void)
int fdm_get_debug_level(JSBSim::FGFDMExec *fdm);
        
//void Initialize(FGInitialCondition *FGIC);
void fdm_initialize(JSBSim::FGFDMExec *fdm, JSBSim::FGInitialCondition *FGIC);
        
//void SetHoldDown(bool hd);
void fdm_set_hold_down(JSBSim::FGFDMExec *fdm, bool hold_down);
        
//bool GetHoldDown(void)
bool fdm_get_hold_down(JSBSim::FGFDMExec *fdm);

//from FGJSBBase class
//string GetVersion()
std::string fdm_get_version(JSBSim::FGFDMExec *fdm);

}           //end extern block
#endif      //end #define WRAPPER_H
