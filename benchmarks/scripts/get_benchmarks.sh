#!/bin/bash

invoke_directory=`pwd`
script_directory=$invoke_directory/`dirname "$0"`

benchmark_template_directory=$script_directory/../template

set -e

cd $benchmark_template_directory

#download and patch sgxbench

wget https://github.com/sgxbench/sgxbench/releases/download/v1.0/sgxbench.tar.gz 
tar xvfz sgxbench.tar.gz
rm sgxbench.tar.gz

patch -s -p1 < $script_directory/sgxbench.patch
chmod +x sgxbench/postprocess.py

#download and patch sgx-nbench

git clone https://github.com/utds3lab/sgx-nbench.git
cd sgx-nbench
git checkout 799f0fcd32d0f0392a3d3cd5b51455c48f121488

cd ..
patch -s -p1 < $script_directory/sgx-nbench.patch
