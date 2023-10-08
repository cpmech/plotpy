#!/bin/bash

set -e

# fake sudo function to be used by docker build
sudo () {
  [[ $EUID = 0 ]] || set -- command sudo "$@"
  "$@"
}

# install dependencies
sudo apt-get update -y && \
sudo apt-get install -y --no-install-recommends \
    g++ \
    gdb \
    git \
    libmetis-dev \
    make \
    python3-matplotlib

# install Intel MKL
bash zscripts/install-intel-mkl-linux.bash
