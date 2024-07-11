#!/bin/bash

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

mkdir -p test/default/spec

cargo test -- --nocapture

TESTS=openapi,openapi_fastapi python test/integration/test.py
