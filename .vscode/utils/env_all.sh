WORKSPACE_FOLDER=$(realpath "$(dirname $0)/../..")
TARGET_PATH=$("$(dirname $0)/env_target_path.sh")
TARGET_FOLDER=$("$(dirname $0)/env_target_folder.sh")
BUILD_PATH=$(realpath "$(dirname ${TARGET_PATH})")

LIVEPROCMON_port=10001
