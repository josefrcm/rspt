#!/bin/bash

export RUST_BACKTRACE=1
cargo build --release
rm render.png
cargo run --release -- --input data/scene.ron --camera data/camera.ron --output render.png --num-samples 1000 --max-bounces 4 --width 1920 --height 1080
#valgrind --tool=callgrind --callgrind-out-file=callgrind-rspt.out target/release/rspt --input data/scene.ron --camera data/camera.ron --output render.png --num-samples 10 --max-bounces 4 --width 1920 --height 1080
#callgrind_annotate callgrind-rspt.out
