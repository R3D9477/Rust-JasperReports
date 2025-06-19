#!/bin/bash
source "$(realpath $(dirname $0)/env_all.sh)"

cd "${WORKSPACE_FOLDER}/.vscode/liveprocmon"

LIVEPROCMON_generatorPath=$(realpath "./liveprocmon_generator.py")
LIVEPROCMON_templatePath=$(realpath "./liveprocmon_cpumem.py_template")

LIVEPROCMON_maxPoints=$1
LIVEPROCMON_collectDelayMs=$2
LIVEPROCMON_generatePlotDelayMs=$3
LIVEPROCMON_updateFrequencyMs=$4
LIVEPROCMON_plotWidthInch=$5
LIVEPROCMON_plotHeightInch=$6

python3 \
    ${LIVEPROCMON_generatorPath} \
    ${LIVEPROCMON_templatePath} \
    ${TARGET_PATH} \
    ${TARGET_FOLDER} \
    ${LIVEPROCMON_port} \
    ${LIVEPROCMON_maxPoints} \
    ${LIVEPROCMON_collectDelayMs} \
    ${LIVEPROCMON_generatePlotDelayMs} \
    ${LIVEPROCMON_updateFrequencyMs} \
    ${LIVEPROCMON_plotWidthInch} \
    ${LIVEPROCMON_plotHeightInch}
