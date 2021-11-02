# Benchmarks

This directory contains scripts to evaluate LVINullify against other mitigation options for sgxbench and sgx-nbench.

# 1 Getting the Benchmarks
run  
`./scripts/get_benchmarks.sh`  
to download and patch the two benchmarks.

# 2 Setting up SDKs and compiling benchmarks
This step will compile the Intel SDK once for the all mitigation options you want to try.
Then, it will compile both benchmarks with the mitigations and the corresponding SDKs.

Two scripts are of note here:  
`./scripts/benchmarks_build.sh` and  
`./scripts/benchmarks_update_template.sh` 

Both take the desired mitigation options as a list of parameters, or all options by default.
Valid options are:

`clang`: build a normally, but with clang, not gcc  
`clang-gs`: build with the LVINullify mitigation  
`clang-lvi-cfi`: build with the Intel control-flow mitigation  
`clang-lvi-opt`: build with the Intel optimized-cut mitigation  
`clang-lvi-full-seses`: build with the SESES mitigation  

`./scripts/benchmarks_build.sh` builds all passed SDKs and benchmarks in one go.

`./scripts/benchmarks_update_template.sh` only builds the selected benchmarks, assuming the appropriate SDKs already exist.

If you have successfully installed the SDK and PSW according to the [Readme](../README.md) in the base directory, you might already have clang-gs SDK installed in SDKs/clang-gs. In this case, you can call `benchmarks_build.sh` without the clang-gs parameter, and just add the benchmarks for it with `benchmarks_update_template.sh clang-gs` later.

# 3 Running Benchmarks

Benchmarks can be started with the same mitigation parameters as previously described.  
The scripts are  
`./scripts/benchmarks_run_nbench.sh` and  
`./scripts/benchmarks_run_sgxbench.sh`  

If you're only interested in a quick check, you can shorten the nbench runtime (see **5 Customization**).

All benchmarks should be run with a fixed, sustainable cpu frequency on a quiet system to guarantee stable results.  
You can use the `cpupower` tool for this (e.g. `sudo cpupower -c all frequency-set -g performance -d 1.9G -u 1.9G`).  
Depending on your CPU and OS, this might be a bit different. You can verify the CPU frequency with `sudo cpupower frequency-info`.  
Ideally, you should also run the benchmarks on isolated CPU cores.  
You can isolate cores by adding the kernel parameter isolcpus=3,7 to the commandline (where 3,7 are hyperthreads on a quad-core in this example).  
The parameters can be set by editing `/etc/default/grub` with sudo and updating grub with `sudo update-grub` and rebooting.

Both run scripts have a variable `isolated_core` that you can set to one of your isolated cores.

# 4 Evaluation

To get the data and figures in the paper from the generated `result_` directories, first convert them to .csv with  
`./scripts/benchmarks_to_csv_nbench.sh` or  
`./scripts/benchmarks_to_csv_sgxbench.sh`  

These scripts *require NUMPY*, `sudo apt-get install python3-numpy`.

Now, apply our high-tech conversion method of opening the csv files (`results.csv`) in the results directory and copy/paste the data into the aptly named `hightech_nbench_plotting.ods` or `hightech_sgxbench_formatting.ods`, which you can do in OpenOffice, LibreOffice or Excel.  
For nbench, make sure to copy the clang result to the appropriate line, as it is the reference for the others.

You can also look at the lfence/gs-relative instruction counts and binary sizes (table 2 in the paper) with `./scripts/analyze_enclaves.sh`.

# 5 Customization

Both run scripts have a variable `isolated_core` to determine which core the benchmark should be run on.  
If you have no isolated core, you can set this to any valid number or remove the taskset command completely.

The sgx-nbench benchmark can be sped up by reducing the number of iterations. Just change the `#define N_RUNS` in line `795` to something else and recompile with `benchmarks_update_template.sh`
