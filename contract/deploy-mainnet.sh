#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near deploy --wasmFile ./target/wasm32-unknown-unknown/release/hello_near.wasm --masterAccount mutations.dapplets.near --accountId mutations.dapplets.near