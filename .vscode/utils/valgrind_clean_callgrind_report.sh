#!/bin/bash
source "$(realpath $(dirname $0)/env_all.sh)"

cd ${BUILD_PATH}

rm -f \
    ${TARGET_PATH}.callgrind \
    ${TARGET_PATH}.callgrind.dot \
    ${TARGET_PATH}.callgrind.svg
