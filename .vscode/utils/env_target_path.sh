#!/bin/bash
WORKSPACE_FOLDER=$(realpath "$(dirname $0)/../..")
cd ${WORKSPACE_FOLDER}

TARGET_NAME=`cargo metadata --no-deps --format-version 1 | jq -r '.packages[0] | .name'`

echo "${WORKSPACE_FOLDER}/target/debug/${TARGET_NAME}"
