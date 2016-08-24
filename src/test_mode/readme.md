#Test Mode Notes

##JSBSim:

###JSBSim Links:
website:  http://jsbsim.sourceforge.net

manual:   http://jsbsim.sourceforge.net/JSBSimReferenceManual.pdf

###JSBSim Script Directory Structure
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
###Notes:
Verify that the installer has placed libJSBSim.so in /usr/local/lib/

If you see an error loading libJSBSim.so*, make sure you add /usr/local/lib to LD_LIBRARY_PATH.

**Note** for debugging, add -DCMAKE_CXX_FLAGS="-O0 -g"

###Debugging the C++ Wrapper
By adding the following line to src/CMakeLists.txt debug symbols can be enabled
`SET(CMAKE_CXX_FLAGS "-O0 -g")`



