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

# build benchmarks
for bench in $benchmarks; do
    sudo rm -r $benchmark_root_directory/$bench
done