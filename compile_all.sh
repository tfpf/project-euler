#! /bin/bash

for source in src/*/*/*.rs
do
    executable=${source%.*}
    if [ ! -f $executable ] || [ $(date -r $source '+%s') -ge $(date -r $executable '+%s') ]
    then
        # Display each compile command as it is executed.
        bash -x -c "rustc -O --out-dir $(dirname $source) $source"
    fi
done
