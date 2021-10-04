#!/bin/bash

# remove old sdk
sudo /opt/intel/sgxsdk/uninstall.sh

# remove old psw
sudo apt remove -y sgx-aesm-service
sudo apt remove -y '^libsgx_*'
