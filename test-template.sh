#!/usr/bin/env bash

set -e

TOOLCHAIN='stable'

SCRIPT_PATH=$(dirname "$0")
TEST_TARGET_DIR=$(mktemp -d)

function setup() {
  mkdir -p $TEST_TARGET_DIR
}

function cleanup() {
  rm -rf $TEST_TARGET_DIR
}

trap cleanup EXIT

setup

cd $TEST_TARGET_DIR

cargo +$TOOLCHAIN generate --verbose --init --path $SCRIPT_PATH -n test_project -d description="This is a test for the cli template"

echo $SCRIPT_PATH
ls $SCRIPT_PATH
pwd
ls
cat README.md

# Make sure correct reamdme is produced
if ! grep -Fxq "# test-project" README.md; then
  echo "ERROR README.md missing the project title"
  exit 1
fi
if ! grep -Fxq "This is a test for the cli template" README.md; then
  echo "ERROR README.md is missing the project description"
  exit 1
fi

# Make sure the correct dependabot config exists
if [ ! -f ".github/dependabot.yml" ]; then
  echo "ERROR .github/dependabot.yml is missing"
  exit 1
fi

# Make sure github workflows configured correctly
if [ -d ".github/workflows" ]; then
  echo "ERROR .github/workflows should not exist"
  exit 1
fi

cargo +$TOOLCHAIN make check-format
cargo +$TOOLCHAIN make clippy
cargo +$TOOLCHAIN make test