# Compiler
To build the compiler for LVI-NULLify execute:
```
./build.sh
```
inside the `compiler` folder.

# Relocator
To build the relocator install rust nightly on your system see [how-to-install-rust](https://www.rust-lang.org/tools/install).

Then run:
```
make
```
inside the `relocator` folder.

# SGX SDK/PSW
We advise to completely remove all SDK and PSW installations from the system before starting the installation.

## Install unpatched SDK
First run:
```
./checkout.sh
```
inside the `sgx-sdk` folder.

Then follow the Intel documentation on how to install only the SDK from the `sgx-sdk/linux-sgx` directory.
And source the resulting environment.

This step creates an unpatched SDK that is necessary for PSW installation.

## Install patched PSW
Then clean the complete repository. We advise to delete the repository and redo the checkout process again.

Afterwards apply the patch:
```
./patch
```
inside the `sgx-sdk` folder.

Now follow the Intel documentation on how to install the PSW from the `sgx-sdk/linux-sgx` directory.

## Install patched SDK
As last step build install the patched SDK from `sgx-sdk/linux-sgx`, following the Intel documentation. 

# Example Enclave
We provide a minimal example inside the `example` folder.


# Warnings
**Warning #1**: We are providing this code as-is. You are responsible for protecting yourself, your property and data, and others from any risks caused by this code. This code may cause unexpected and undesirable behavior to occur on your machine.

**Warning #2**: This code is only for testing purposes. Do not run it on any production systems. Do not run it on any system that might be used by another person or entity.
