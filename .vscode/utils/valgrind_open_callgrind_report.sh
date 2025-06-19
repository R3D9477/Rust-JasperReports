#!/bin/bash
source "$(realpath $(dirname $0)/env_all.sh)"

cd ${BUILD_PATH}

valgrind \
    --tool=callgrind \
    --dump-instr=yes \
    --collect-jumps=yes \
    --callgrind-out-file=${TARGET_PATH}.callgrind \
    ${TARGET_PATH}

gprof2dot \
    --format=callgrind \
    --output=${TARGET_PATH}.callgrind.dot \
    ${TARGET_PATH}.callgrind

code -r ${TARGET_PATH}.callgrind.dot
