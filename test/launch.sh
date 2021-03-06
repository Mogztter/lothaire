#!/bin/bash

for d in vagrant/*; do
    if [ -d $d ]; then
        export VAGRANT_CWD="./${d}/"
        vagrant destroy -f
        vagrant up
    fi
done
