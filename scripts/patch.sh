#!/bin/sh

cd ../linux-sgx
patch -p1 < ../scripts/sgx-sdk.patch

echo "patched sdk"
