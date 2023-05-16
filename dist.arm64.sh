#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

git pull
yarn build --target aarch64-unknown-linux-gnu
mv *.linux-arm64-gnu.node npm/linux-arm64-gnu
out=npm/linux-arm64-gnu/package.json
if ! [ -x "$(command -v sponge)" ]; then
  apt-get install -y moreutils
fi
jq -c ".version = $(cat package.json|jq .version)" $out | sponge $out
cd npm/linux-arm64-gnu
npm publish --access=public
git checkout .
