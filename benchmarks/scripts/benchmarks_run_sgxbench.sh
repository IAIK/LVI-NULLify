#!/bin/bash

isolated_core=3

if [ $# -eq 0 ]; then
    benchmarks="clang clang-gs clang-lvi-cfi clang-lvi-opt clang-lvi-full-seses"
else
    benchmarks=$@
fi

# directories
invoke_directory=`pwd`
script_directory=$invoke_directory/`dirname "$0"`

benchmark_root_directory=$script_directory/..

date=`date +"%F_%H.%M"`
result_dir=$benchmark_root_directory/result_sgxbench_$date

set -e
mkdir $result_dir

cat /proc/cpuinfo > $result_dir/meta_cpuinfo
cat /proc/cmdline > $result_dir/meta_cmdline
lsb_release -a > $result_dir/meta_lsbrelease
uname -r > $result_dir/meta_kernel

for bench in $benchmarks; do
    echo "Running: $bench"
    cd $benchmark_root_directory/$bench/benchmark/sgxbench/
    
    #2: // empty function, cold cache
    #10: // empty ocall, cold cache
    #26: // oinout2
    #31: // enc read
    #32: // enc write
    #204: // test enclave destroy, cold cache

    nrs="2 10 26 31 32 204"

    set +e
    rm -r ./results
    set -e
    mkdir ./results
    
    for nr in $nrs; do
        echo "-running $nr"
        taskset -c $isolated_core ./sgxbench $nr > /dev/null
    done
    ./postprocess.py
    cp ./results/results.csv $result_dir/result_$bench.csv
done
