#!/bin/bash
source "$(realpath $(dirname $0)/env_all.sh)"

cd "${WORKSPACE_FOLDER}/.vscode/liveprocmon/generated"

python3 -m "http.server" ${LIVEPROCMON_port}
