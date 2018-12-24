export RUST_BACKTRACE=1

cargo build --release
time target/release/rusty_ray -s 10 -i data/scene.json -o foo.tga

# cargo build
# time target/debug/rusty_ray -s 100 -i data/scene.json -o foo.tga
