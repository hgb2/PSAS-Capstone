#Test Mode Notes

##JSBSim:
website:  http://jsbsim.sourceforge.net

manual:   http://jsbsim.sourceforge.net/JSBSimReferenceManual.pdf

##JSBSim Script Directory Structure
```
test_mode / jsbsim / aircraft
                   / engine
                   / scripts
                   / systems
                   / testlogs
```

##Compiling JSBSim on Linux

###Prep
```
sudo apt-get update
sudo apt-get install cmake      #install cmake
```

###Compilation (as libJSBSim.so)
```
wget -O jsbsim.tar.gz http://jsbsim.cvs.sourceforge.net/viewvc/jsbsim/?view=tar
tar -xvf jsbsim.tar.gz
cd jsbsim/JSBSim/
cmake -DBUILD_SHARED_LIBS=TRUE
make
sudo make install
```
###follow up:
verify that the installer has placed libJSBSim.so in /usr/local/lib/

if you see an error loading libJSBSim.so*, make sure you add /usr/local/lib to LD_LIBRARY_PATH. 
**Note** for debugging, add -DCMAKE_CXX_FLAGS="-O0 -g"
##Compiling JSBSim on Windows

###Downloads:
    MSYS2 installer from https://msys2.github.io/
    TDM GCC from http://tdm-gcc.tdragon.net/
    CMake (with GUI) https://cmake.org/download/

###Install Directions
1) Follow the directions on https://msys2.github.io/ to install MSYS2.

2) Install TDM GCC, only 64-bit tested so far. Make sure you update your PATH variable for both Windows and Msys. There is a .bashrc file in the home directory of the Msys installation where environment variables can be exported.

3) Install CMake and Rust with the GNU ABI

4) Build JSBSIM with cmake-generated makefiles of type "MSYS2 Makefiles" as a shared library.

5) Edit CMakeLists.txt:
a) Update the location of JSBSIM includes, library and library search directory. Note: for C:\Foo\file.txt, the msys path would be /c/Foo/file.txt
    
b) Set the C++ STL installation for TDM. For example: /c/TDM-GCC-64/lib/gcc/x86_64-w64-mingw32/5.1.0/include/c++

c) Add another include directive pointing to the containing "bits/c++config.h". For example: /c/TDM-GCC-64/lib/gcc/x86_64-w64-mingw32/5.1.0/include/c++/x86_64-w64-mingw32

6) Backup, and replace the version of libstdc++.a in the Rust compile toolchain installation. For example: C:\rust\lib\rustlib\x86_64-pc-windows-gnu\lib\libstdc++.a with the version from the TDM-GCC installation: For example: C:\dev\TDM-GCC-64\lib\gcc\x86_64-w64-mingw32\5.1.0\libstdc++.a

7) run `cargo run` from the test directory.

###Debugging the C++ Wrapper
By adding the following line to src/CMakeLists.txt debug symbols can be enabled
`SET(CMAKE_CXX_FLAGS "-O0 -g")`


##Function Reference
```
JSBSim constructor
Rust:     fn fdm_create()->*mut FDM;
C ABI:    JSBSim::FGFDMExec* fdm_create();

JSBSim destructor                                      //these are not implemented
Rust:     fn fdm_close(fdm: *mut FDM);                 //definition provided for reference
C ABI:    void fdm_close(JSBSim::FGFDMExec *fdm);      //definition provided for reference

JSBSim functions                                       //from JSBSim::FGFDMExec
Rust:     fn fdm_run(fdm: *mut FDM)->bool;
C ABI:    bool fdm_run(JSBSim::FGFDMExec *fdm);

Rust:     fn fdm_run_ic(fdm: *mut FDM)->bool;
C ABI:    bool fdm_run_ic(JSBSim::FGFDMExec *fdm);

Rust:     fn fdm_load_script(fdm: *mut FDM, script_name: *const libc::c_char, delta_t: f64, init_file: *const libc::c_char)->bool;
C ABI:    bool fdm_load_script(JSBSim::FGFDMExec *fdm, const char* script_name, double delta_t, const char* init_file);

Rust:     fn fdm_set_aircraft_path(fdm: *mut FDM, aircraft_path: *const libc::c_char)->bool;
C ABI:    bool fdm_set_aircraft_path(JSBSim::FGFDMExec *fdm, const char* aircraft_path);

Rust:     fn fdm_set_engine_path(fdm: *mut FDM, engine_path: *const libc::c_char)->bool;
C ABI:    bool fdm_set_engine_path(JSBSim::FGFDMExec *fdm, const char* engine_path);

Rust:     fn fdm_set_systems_path(fdm: *mut FDM, systems_path: *const libc::c_char)->bool;
C ABI:    bool fdm_set_systems_path(JSBSim::FGFDMExec *fdm, const char* systems_path);

Rust:     fn fdm_set_root_dir(fdm: *mut FDM, root_dir: *const libc::c_char);
C ABI:    void fdm_set_root_dir(JSBSim::FGFDMExec *fdm, const char* root_dir);

Rust:     fn fdm_get_property_double(fdm: *mut FDM, property: *const libc::c_char)->f64;
C ABI:    double fdm_get_property_double(JSBSim::FGFDMExec *fdm, const char* property);

Rust:     fn fdm_set_property_double(fdm: *mut FDM, property: *const libc::c_char, value: f64);
C ABI:    void fdm_set_property_double(JSBSim::FGFDMExec *fdm, const char* property, double value);
```



