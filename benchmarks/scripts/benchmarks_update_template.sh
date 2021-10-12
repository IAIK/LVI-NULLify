#!/bin/bash

if [ $# -eq 0 ]; then
    benchmarks="clang clang-gs clang-lvi-cfi clang-lvi-opt  clang-lvi-full-seses"
else
    benchmarks=$@
fi

# directories
invoke_directory=`pwd`
script_directory=$invoke_directory/`dirname "$0"`

benchmark_root_directory=$script_directory/..
compiler_directory=$script_directory/../../compiler
sdk_root_directory=$script_directory/../../SDKs

# build benchmarks
for bench in $benchmarks; do

    cd $invoke_directory

    benchmark_directory=$benchmark_root_directory/$bench
    sudo rm -r $benchmark_directory/benchmark

    # exit if any command failes ... otherwise we could nuke our own it repo
    set -e

    mkdir -p $benchmark_directory/

    # if clang-gs set dr
    if [ "$bench" = "clang-gs" ]; then
        dr="use"
    else
        dr=""
    fi

    # copy template
    cp -R $benchmark_root_directory/template $benchmark_directory/benchmark

    # build sgx-nbench
    cd $benchmark_directory/benchmark/sgx-nbench

    make SGX_PRERELEASE=1 SGX_DEBUG=0  SGX_SDK=$sdk_root_directory/$bench/sgxsdk DR=$dr TR_CC=$compiler_directory/$bench TR_CXX=$compiler_directory/$bench++ clean all

    # build sgxbench
    cd $benchmark_directory/benchmark/sgxbench

    autoconf
    ./configure CFLAGS='-O3' CXXFLAGS='-O3' --with-sgxsdk=$sdk_root_directory/$bench/sgxsdk --with-sgx-build=prerelease
    make SGX_PRERELEASE=1 SGX_DEBUG=0 SGX_SDK=$sdk_root_directory/$bench/sgxsdk DR=$dr TR_CC=$compiler_directory/$bench TR_CXX=$compiler_directory/$bench++ clean all

    set +e

done