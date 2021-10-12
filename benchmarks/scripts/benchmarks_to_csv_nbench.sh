#!/bin/bash

if [ $# -eq 0 ]; then
    echo "specify result directory" 
    exit -1
fi

result_dir=$@
result_csv=$result_dir/results.csv

cd $result_dir
result_files=`find . -name "result_*.log" | tr '\n' ' '`
cd ..

echo "NAME;NUMERIC SORT;NUMERIC SORT SD;STRING SORT;STRING SORT SD;BITFIELD;BITFIELD SD;FP EMULATION;FP EMULATION SD;FOURIER;FOURIER SD;ASSIGNMENT;ASSIGNMENT SD;IDEA;IDEA SD;HUFFMAN;HUFFMAN SD;NEURAL NET;NEURAL NET SD;LU DECOMPOSITION;LU DECOMPOSITION SD" > $result_csv

for result_file in $result_files; do
    bench_name=`echo $result_file | sed 's/.\/result_\(.*\).log/\1/'`

    printf "$bench_name; " >> $result_csv
    cat $result_dir/$result_file | tail -n +4 | head -n -1 | tr -s ' ' | cut -d":" -f2-3 | tr ':' ';' | paste -sd ";" | tr -d '\n'  >> $result_csv
    echo "" >> $result_csv
done

cat $result_csv