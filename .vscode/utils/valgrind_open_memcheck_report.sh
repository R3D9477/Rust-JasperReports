#!/bin/bash
source "$(realpath $(dirname $0)/env_all.sh)"

cd ${BUILD_PATH}

valgrind \
    --tool=memcheck \
    --leak-check=full \
    --show-leak-kinds=all \
    --track-origins=yes \
    --verbose \
    --log-file=${TARGET_PATH}.valgrind \
    ${TARGET_PATH}

code -r ${TARGET_PATH}.valgrind
