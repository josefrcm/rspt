#!/bin/bash

export RUST_BACKTRACE=1
cargo build --release
rm render.png
target/release/rspt --input data/scene.json --camera data/camera.json --output render.png --num-samples 1000 --max-bounces 4 --width 1920 --height 1080
