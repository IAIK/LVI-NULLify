#!/bin/sh

git clone https://github.com/intel/linux-sgx.git
cd linux-sgx
git checkout 60d36e0de7055e8edd2fe68693b3c39f3f10fd3c
patch -p1 < ../sgx-sdk.patch
