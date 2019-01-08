set RUST_BACKTRACE=1
cargo build --release
del target\release\render.tga
target\release\rusty_ray -s 100 -i data\scene.json -c data\camera.json -o target\release\render.tga
