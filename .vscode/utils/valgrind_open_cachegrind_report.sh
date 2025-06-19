#!/bin/bash
source "$(realpath $(dirname $0)/env_all.sh)"

cd ${BUILD_PATH}

valgrind \
    --tool=cachegrind \
    --cachegrind-out-file=${TARGET_PATH}.cachegrind \
    ${TARGET_PATH}

cg_annotate ${TARGET_PATH}.cachegrind \
    $(echo \
        $( find ${workspaceFolder} \
            -name '*.rs' -not -path "${BUILD_PATH}/*" -o \
            -name '*.cpp' -not -path "${BUILD_PATH}/*" -o \
            -name '*.c' -not -path "${BUILD_PATH}/*" -o \
            -name '*.cc' -not -path "${BUILD_PATH}/*" -o \
            -name '*.h' -not -path "${BUILD_PATH}/*" -o \
            -name '*.hpp' -not -path "${BUILD_PATH}/*" )
    ) > ${TARGET_PATH}.cachegrind.valgrind

code -r ${TARGET_PATH}.cachegrind.valgrind 
