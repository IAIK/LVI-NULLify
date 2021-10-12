#!/bin/bash

if [ $# -eq 0 ]; then
    benchmarks="clang clang-gs clang-lvi-cfi clang-lvi-opt clang-lvi-full-seses"
else
    benchmarks=$@
fi

# directories
invoke_directory=`pwd`
script_directory=$invoke_directory/`dirname "$0"`
benchmark_root_directory=$script_directory/..
sdk_root_directory=$script_directory/../../SDKs

for bench in $benchmarks; do    
    echo "$bench:"

    cd $benchmark_root_directory/$bench/benchmark/sgx-nbench/
    n_lfence=`objdump Enclave.so -dC | grep "lfence" | wc -l` 
    n_gs=`objdump Enclave.so -dC | grep "%gs" | wc -l`
    n_ret=`objdump Enclave.so -dC | grep "ret" | wc -l`
    size=`ls -l Enclave.so | cut -d " " -f 5`
    echo "-nbench:         lfences=$n_lfence gs=$n_gs size=$size rets=$n_ret"

    cd $benchmark_root_directory/$bench/benchmark/sgxbench/Enclave
    n_lfence=`objdump Enclave.so -dC | grep "lfence" | wc -l` 
    n_gs=`objdump Enclave.so -dC | grep "%gs" | wc -l`
    n_ret=`objdump Enclave.so -dC | grep "ret" | wc -l`
    size=`ls -l Enclave.so | cut -d " " -f 5`
    echo "-sgxbench:       lfences=$n_lfence gs=$n_gs size=$size rets=$n_ret"

    cd $sdk_root_directory/$bench/sgxsdk/lib64/
    n_lfence=`objdump libsgx_trts.a -dC | grep "lfence" | wc -l` 
    n_gs=`objdump libsgx_trts.a -dC | grep "%gs" | wc -l`
    size=`ls -l libsgx_trts.a | cut -d " " -f 5` 
    echo "-libsgx_trts.a:  lfences=$n_lfence gs=$n_gs size=$size"

    cd $sdk_root_directory/$bench/sgxsdk/lib64/
    n_lfence=`objdump libsgx_tstdc.a -dC | grep "lfence" | wc -l` 
    n_gs=`objdump libsgx_tstdc.a -dC | grep "%gs" | wc -l`
    size=`ls -l libsgx_tstdc.a | cut -d " " -f 5`
    echo "-libsgx_tstdc.a: lfences=$n_lfence gs=$n_gs size=$size"

    cd $sdk_root_directory/$bench/sgxsdk/lib64/
    n_lfence=`objdump libsgx_tcxx.a -dC | grep "lfence" | wc -l` 
    n_gs=`objdump libsgx_tcxx.a -dC | grep "%gs" | wc -l`
    size=`ls -l libsgx_tcxx.a | cut -d " " -f 5`
    echo "-libsgx_tcxx.a:  lfences=$n_lfence gs=$n_gs size=$size"


done
