#!/bin/bash

script_directory=`pwd`
sdk_directory=$script_directory/../SDKs/clang-gs/
sgx_directory=$script_directory/../linux-sgx

# remove old patched sdk
sudo $sdk_directory/sgxsdk/uninstall.sh

cd $sgx_directory
make sdk_install_pkg

printf "no\n$sdk_directory\n" | sudo linux/installer/bin/sgx_linux_x64_sdk_2.10.100.2.bin

