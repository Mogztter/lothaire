#!/bin/bash

export VAGRANT_CWD="./vagrant/centos7/"
vagrant up
vagrant ssh -c "(cd /lothaire/src && cargo test -- --nocapture)"
