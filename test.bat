set RUST_BACKTRACE=1
cargo build --release
del render.png
target\release\rspt -s 100 -i data\scene.json -c data\camera.json -o render.png
