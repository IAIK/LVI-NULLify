#!/bin/bash

# directories
script_directory=`pwd`
sgx_directory=$script_directory/../linux-sgx

set -e


#uninstall sdk/psw
$script_directory/uninstall_psw_sdk.sh

echo "DONE uninstalling default sdk/psw"

#install unmodified sdk to /opt/intel
$script_directory/install_unmodified_sdk.sh

echo "DONE installing unmodified sdk"

#clean up
$script_directory/install_patched_psw.sh
echo "DONE installing patched psw"

#install modified sdk to "SDKS"
$script_directory/install_patched_sdk.sh

echo "DONE installing patched sdk"



echo "add the following to /etc/apt/sources.list"
echo "deb [trusted=yes arch=amd64] file:$sgx_directory/linux/installer/deb/sgx_debian_local_repo bionic main"
echo "change bionic to match your ubuntu release"
echo "then run:"
echo "sudo apt update"
echo "sudo apt install sgx-aesm-service '^libsgx_*'"