#!/bin/sh

# llvm 11 for mitigations
git clone --branch llvmorg-11.0.0-rc5 --depth 1 https://github.com/llvm/llvm-project.git llvm11
cd llvm11 && mkdir build
patch -p1 < ../llvm-lvi-nullify.patch
cd build
cmake ../llvm -G Ninja -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_PROJECTS=clang -DENABLE_EXPERIMENTAL_NEW_PASS_MANAGER=true
ninja

cd ../..

# optimized plugin
git clone https://github.com/intel/lvi-llvm-optimization-plugin.git
cd lvi-llvm-optimization-plugin
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=RELEASE ..
make
