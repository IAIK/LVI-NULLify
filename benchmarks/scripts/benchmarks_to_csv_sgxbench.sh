#!/bin/bash

if [ $# -eq 0 ]; then
    echo "specify result directory" 
    exit -1
fi

result_dir=$@
result_csv=$result_dir/results.csv

cd $result_dir
result_files=`find . -name "result_*.csv" | tr '\n' ' '`
cd ..

echo "NAME;empty func;empty func SD;empty func N;empty ocall;empty ocall SD;empty ocall N;ocall inout;ocall inout SD;ocall inout N;enc read;enc read SD;enc read N;enc write;enc write SD;enc write N;einit edestroy; einit edestroy SD; einit edestroy N" > $result_csv

for result_file in $result_files; do
    bench_name=`echo $result_file | sed 's/.\/result_\(.*\).csv/\1/'`
    echo $bench_name

    printf "$bench_name;" >> $result_csv
    cat $result_dir/$result_file >> $result_csv
    #printf "\n" >> $result_csv
done