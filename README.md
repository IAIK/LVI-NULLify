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
We advice to completely remove all SDK and PSW installations from the system before starting the installation.

## Install unpatched SDK
First run:
```
./checkout.sh
```
inside the `sgx-sdk` folder.

Then follow the Intel documentation on how to install only the SDK from the `sgx-sdk/linux-sgx` directory.
And source the resulting environment.

## Install patched PSW
Then clean the complete repository. We advice to delete the repository and redo the checkout process again.

Afterwards apply the patch:
```
./patch
```
inside the `sgx-sdk` folder.

Again follow the Intel documentation on how to install the PSW from the `sgx-sdk/linux-sgx` directory.

## Install patched SDK
As last step build install the patched SDK from `sgx-sdk/linux-sgx`, following the Intel documentation. 

# Example Enclave
We provide a minimal example inside the `example` folder.