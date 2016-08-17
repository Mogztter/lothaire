#!/bin/bash

for d in vagrant/*; do
    if [ -d $d ]; then
        export VAGRANT_CWD="./${d}/"
        vagrant up
        vagrant ssh -c "(cd /lothaire/src && cargo test)"
    fi
done
