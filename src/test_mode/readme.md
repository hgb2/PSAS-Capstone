#Test Mode Notes:

##Compiling JSBSim

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
sudo make
sudo make install
```

###follow up:
verify that the installer has placed libJSBSim.so in /usr/local/lib/.

###jsbsim links:
website:  http://jsbsim.sourceforge.net

manual:   http://jsbsim.sourceforge.net/JSBSimReferenceManual.pdf
