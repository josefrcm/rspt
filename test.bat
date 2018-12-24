set RUST_BACKTRACE=1
cargo build --release
del render.tga
target\release\rusty_ray -s 10 -i data\scene.json -c data\camera.json -o target\release\render.tga
