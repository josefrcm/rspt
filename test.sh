#!/bin/bash

export RUST_BACKTRACE=1
cargo build --release
rm render.tga
time target/release/rusty_ray -s 100 -i data/scene.json -c data/camera.json -o target/release/render.tga
