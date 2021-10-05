# LVINullify

This is the PoC implementation for the USENIX 2022 paper

[Repurposing Segmentation as a Practical LVI-NULL Mitigation in SGX](https://ginerlukas.com/publications/papers/lvinull.pdf) by Lukas Giner, Andreas Kogler, Claudio Canella, Michael Schwarz and Daniel Gruss.

This readme walks you through compiling a modified clang compiler, an extra relocator, the SGX driver, a modified SGX PSW and a modified and unmodified SGX SDK.  
The modified compiler, PSW, SDK and the relocator comprise LVINullify, which allows you to build and run LVI-Null hardened SGX enclaves.

The total install time should be around 1h-2h.

# 1 Compiler
To build the compiler for LVI-NULLify, install the prerequisites for llvm (e.g. cmake and ninja-build) and run  
`./build.sh` 
inside the `compiler` folder.

If the build fails or hangs (e.g. not enough ram), you can try to restart with `ninja clang` in the `llvm11/build` directory.

# 2 Relocator
To build the relocator, install rust nightly on your system see [how-to-install-rust](https://www.rust-lang.org/tools/install).

Then run:  
`make`  
inside the `relocator` folder.

# 3 SGX Driver
Install the SGX driver according to https://github.com/intel/linux-sgx-driver .

# 4 SGX SDK/PSW

If you have no need to preserve an old PSW or SDK and defaults for everything are fine (and you're feeling lucky), you can skip all steps in **4** by running  
`setup.sh`  
from within the `scripts` folder.
You still have to manually update your source.list file and install the packages, the script prints these instructions at the end.


Continue on for (partially) manual install:  

We advise to completely remove all SDK and PSW installations from the system before starting the installation.
If you have previously installed a version of SGX that uses local packages, you may uninstall sdk and psw with  
`./uninstall_psw_sdk.sh`

Please install all prerequisites for your system according to https://github.com/intel/linux-sgx/blob/master/README.md .

## 4.1 Install unpatched SDK

This step creates an unpatched SDK that is necessary for PSW installation.

Otherwise, from within the `scripts` folder:

### Automatic:
If you want to install this to the default location, /opt/intel (recommended), you can run `./install_unmodified_sdk.sh`.
### Manual:
First run:  
`./checkout.sh`

Follow the Intel documentation on how to install only the SDK from the `linux-sgx` directory.

Now, source the resulting environment (e.g. `source /opt/intel/sgxsdk`).


## 4.2 Install patched PSW
### Automatic:
`install_patched_psw.sh`  
You still have to manually update your source.list file and install the packages, the script prints these instructions at the end.
### Manual:
Clean the complete repository. We advise to delete the repository and redo the `./checkout.sh` process again.

Afterwards apply the patch:  
`./patch`  
inside the `scripts` folder.

Now follow the Intel documentation on how to install the PSW from the `linux-sgx` directory.

## 4.3 Install patched SDK
### Automatic:
`install_patched_sdk.sh`  
### Manual:
As the last step, build install the patched SDK from `linux-sgx`, following the Intel documentation.
You may want to install this to a different folder (i.e. not /opt/intel), for compatibilty with the PoCs we recommend `SDKs/clang-gs` (in this directory).

# 5 PoC Enclaves
We provide LVI-Null PoCs inside the `POC` folder, see the readme in there.

# Warnings
**Warning #1**: We are providing this code as-is. You are responsible for protecting yourself, your property and data, and others from any risks caused by this code. This code may cause unexpected and undesirable behavior to occur on your machine.

**Warning #2**: This code is only for testing purposes. Do not run it on any production systems. Do not run it on any system that might be used by another person or entity.
