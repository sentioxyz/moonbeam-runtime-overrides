#!/bin/bash

SPEC_VERSION=$1
GIT_REF=${2:-"runtime-$SPEC_VERSION"}

if [[ "$GIT_REF" == "runtime-$SPEC_VERSION" ]]; then
  GIT_DEP_REF="--rev $GIT_REF"
else
  GIT_DEP_REF="--branch $GIT_REF"
fi

if [[ "$SPEC_VERSION" == "local" ]]; then
  MOONBEAM_PATH="../../"
else
  MOONBEAM_PATH="tmp/moonbeam"

  # Get moonbeam repository snapshot
  echo "Get moonbeam snapshot..."
  rm -rf tmp
  mkdir tmp
  git clone https://github.com/PureStake/moonbeam --depth 1 -b $GIT_REF $MOONBEAM_PATH
fi

# Copy relevant files
echo "Copy relevant files and folders..."
mkdir -p tracing/$SPEC_VERSION/runtime
cp $MOONBEAM_PATH/Cargo.toml tracing/$SPEC_VERSION/Cargo.toml
cp $MOONBEAM_PATH/Cargo.lock tracing/$SPEC_VERSION/Cargo.lock
cp $MOONBEAM_PATH/rust-toolchain tracing/$SPEC_VERSION/rust-toolchain
cp -r $MOONBEAM_PATH/runtime/common tracing/$SPEC_VERSION/runtime/
cp -r $MOONBEAM_PATH/runtime/moon* tracing/$SPEC_VERSION/runtime/

# Remove irrelevant files
rm -rf tracing/$SPEC_VERSION/runtime/relay-encoder

echo "Run migration script"
cd scripts
cargo run -q --bin migrate-imported -- --dir ../tracing/$SPEC_VERSION --repo "https://github.com/PureStake/moonbeam" $GIT_DEP_REF
cd ..

echo "Done !"