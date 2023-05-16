#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

git pull
yarn build --target aarch64-unknown-linux-gnu
mv *.linux-arm64-gnu.node npm/linux-arm64-gnu
jq -i ".version = $(cat package.json|jq .version)" npm/linux-arm64-gnu/package.json
cd npm/linux-arm64-gnu
npm publish --access=public
git checkout .
