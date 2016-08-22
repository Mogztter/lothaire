#!/bin/bash

export VAGRANT_CWD="./vagrant/debian8/"
vagrant up
vagrant ssh -c "(cd /lothaire/src && cargo test)"
