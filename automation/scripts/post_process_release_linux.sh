#! /bin/bash
# This script will bundle the game under Linux for publication
set -ex

export PROD_TMP=./target/production_tmp

rm -rf $PROD_TMP
mkdir -p $PROD_TMP

mv ./target/release/desktop_wrapper $PROD_TMP/ldjam50-linux-x64
chmod +x $PROD_TMP/ldjam50-linux-x64