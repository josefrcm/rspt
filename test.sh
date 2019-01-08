#!/bin/bash

export RUST_BACKTRACE=1
cargo build --release
rm target/release/render.png
time target/release/rspt -s 100 -i data/scene.json -c data/camera.json -o target/release/render.hpngdr
