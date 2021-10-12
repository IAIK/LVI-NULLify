#!/bin/bash

if [ $# -eq 0 ]; then
    benchmarks="clang clang-gs clang-lvi-cfi clang-lvi-opt clang-lvi-full-seses"
else
    benchmarks=$@
fi


# directories
invoke_directory=`pwd`
bench_script_directory=$invoke_directory/`dirname "$0"`

script_directory=$bench_script_directory/../../scripts
benchmark_root_directory=$bench_script_directory/..
sdk_root_directory=$bench_script_directory/../../SDKs
sgx_directory=$bench_script_directory/../../linux-sgx
compiler_directory=$bench_script_directory/../../compiler

# build benchmarks
for bench in $benchmarks; do

    cd $invoke_directory

    sdk_directory=$bench_script_directory/../../SDKs/$bench
    benchmark_directory=$benchmark_root_directory/$bench
    sudo rm -r $benchmark_directory
    mkdir $benchmark_directory
    mkdir $sdk_directory

    # exit if any command fails ... otherwise we could nuke our own repo
    set -e

    rm -r -f $sgx_directory
    $script_directory/checkout.sh

    cd $sgx_directory

    # if clang-gs we need a different branch
    if [ "$bench" = "clang-gs" ]; then
        patch -p1 < $script_directory/sgx-sdk.patch
    else
        patch -p1 < $script_directory/clang.patch
    fi

    ./download_prebuilt.sh

    make TR_CC=$compiler_directory/$bench TR_CXX=$compiler_directory/$bench++ sdk_install_pkg_no_mitigation

    # install sdk locally
    cd $sdk_directory
    printf 'yes\n' | sudo $sgx_directory/linux/installer/bin/sgx_linux_x64_sdk_2.10.100.2.bin


    cd $bench_script_directory
    ./benchmarks_update_template.sh $bench

    set +e

done
