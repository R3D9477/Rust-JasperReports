#!/bin/bash
source "$(realpath $(dirname $0)/env_all.sh)"

cd ${BUILD_PATH}

rm -f \
    ${TARGET_PATH}.cachegrind \
    ${TARGET_PATH}.cachegrind.valgrind
