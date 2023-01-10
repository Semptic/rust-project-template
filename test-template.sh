#!/usr/bin/env bash

set -e

TOOLCHAIN='stable'

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
TEST_TARGET_DIR=$(mktemp -d)

echo $TEST_TARGET_DIR

function setup() {
  mkdir -p $TEST_TARGET_DIR
}

function cleanup() {
  rm -rf $TEST_TARGET_DIR
}

trap cleanup EXIT

setup

cd $TEST_TARGET_DIR

cargo +$TOOLCHAIN generate --init --path $SCRIPT_DIR -n test_project -d description="This is a test for the cli template"

cargo +$TOOLCHAIN make check-format
cargo +$TOOLCHAIN make clippy
cargo +$TOOLCHAIN make test
cargo +$TOOLCHAIN outdated --exit-code 1