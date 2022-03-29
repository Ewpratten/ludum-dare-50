#! /bin/bash
set -ex

export PYTHONPATH=$(pwd)/automation:$PYTHONPATH
export LD50_PROJECT_ROOT=$(pwd)
python3 -m anim_stitcher