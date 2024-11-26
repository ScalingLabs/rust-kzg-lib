#!/bin/bash

set -e

print_msg () {
    echo "[*]" "$1"
}



###################### cloning c-kzg-4844 ###############

print_msg "Removing existing c-kzg-4844"
rm -rf c-kzg-4844

print_msg "Cloning c-kzg-4844"
git clone https://github.com/ethereum/c-kzg-4844.git
cd c-kzg-4844 || exit
git -c advice.detachedHead=false checkout "$C_KZG_4844_GIT_HASH"
git submodule update --init

print_msg "Applying patches and building blst"
cd src
export CFLAGS="-Ofast -fno-builtin-memcpy -fPIC -Wall -Wextra -Werror"
make blst
unset CFLAGS
cd ..

##################### cleaning up ################

print_msg "Cleaning up"
rm -rf c-kzg-4844
