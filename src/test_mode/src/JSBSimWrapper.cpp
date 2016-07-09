#include "JSBSimWrapper.h"

void wrapper_init()
{ 
	JSBSim::FGFDMExec FDMExec;
	printf("Started JSBSim\n");
	//FDMExec.LoadScript("run.xml");
	//FDMExec.Run();
}

void wrapper_update()
{

}

void wrapper_terminate()
{

}

