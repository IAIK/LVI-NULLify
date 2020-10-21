#!/bin/sh

cd linux-sgx
patch -p1 < ../sgx-sdk.patch
