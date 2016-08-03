#Test Mode Notes:

###JSBSim Script Directory Structure
test_mode / jsbsim / aircraft

                   / engine
                   
                   / scripts
                   
                   / systems


##Compiling JSBSim on Linux

###prep
```
sudo apt-get update
sudo apt-get install cmake      //install cmake
```

###jsbsim:  compiling as a dynamic (.so) library
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

###jsbsim links:
website:  http://jsbsim.sourceforge.net

manual:   http://jsbsim.sourceforge.net/JSBSimReferenceManual.pdf



###Compiling JSBSim on Windows

Downloads:
    MSYS2 installer from https://msys2.github.io/
    TDM GCC from http://tdm-gcc.tdragon.net/
    CMake (with GUI) https://cmake.org/download/

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

