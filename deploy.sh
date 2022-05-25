
#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@$1 
readonly TARGET_PATH=/home/pi/Documents/rasp_fi
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/rasp_fi

cargo build --release --target=${TARGET_ARCH}
pscp ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}