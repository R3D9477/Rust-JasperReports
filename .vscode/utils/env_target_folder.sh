#!/bin/bash
WORKSPACE_FOLDER=$(realpath "$(dirname $0)/../..")
cd ${WORKSPACE_FOLDER}

echo "${WORKSPACE_FOLDER}/target/debug"
